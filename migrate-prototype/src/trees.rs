/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use html_to_markdown_rs::convert;

use reqwest::Client;
use sqlx::{
    MySqlExecutor, PgPool, QueryBuilder, query_as,
    types::chrono::{DateTime, NaiveDate, Utc},
};

use crate::{insert_img_in_db, recover_string, replace_picture_link};

#[derive(Debug)]
pub struct Boom {
    boom_id: u32,
    timestamp: DateTime<Utc>,
    user_id: u32,
    laatst_gewijzigd: NaiveDate,
    gepubliceerd: Option<i8>,
    laatst_bezocht: Option<NaiveDate>,
    titel: String,
    boek: Option<String>,
    deel: Option<String>,
    hoofdstuk: Option<String>,
    opgavenummer: Option<u8>,
    omschrijving: Option<String>,
    opgavetekst: Option<String>,
    plaatje: Option<String>,
    orientatiefase: Option<String>,
    planfase: Option<String>,
    afrondfase: Option<String>,
    aantal_bezoeken: u32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeStructure {
    pub root_nodes: Vec<RootNode>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RootNode {
    pub label: String,
    pub children: Vec<Node>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub card_id: i32,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct Tree {
    pub id: i32,
    pub owner_id: i32,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub published: bool,
    pub times_visited: i64,
    pub last_visited: Option<DateTime<Utc>>,
    pub title: String,
    pub description: String,
    pub book_chapter: String,
    pub book_exercise: String,
    pub assignment_text: String,
    pub structure: TreeStructure,
    pub language_code: String,
    pub subject_id: Option<i32>,
    pub book_isbn: Option<i64>,
    pub tags: Vec<String>,
}

pub async fn migrate_trees(
    mysql_db: impl MySqlExecutor<'_>,
    postgres_db: &PgPool,
    client: &Client,
    api_base_url: String,
) {
    let bomen = query_as!(
        Boom,
        "SELECT boom_id, timestamp, user_id, laatst_gewijzigd, gepubliceerd, 
        aantal_bezoeken as `aantal_bezoeken: u32`, laatst_bezocht, titel, boek, 
        deel, hoofdstuk, opgavenummer, omschrijving, opgavetekst, plaatje, 
        orientatiefase, planfase, afrondfase
        FROM `bomen`"
    )
    .fetch_all(mysql_db)
    .await
    .unwrap();

    let mut new_trees = vec![];

    for boom in &bomen {
        let tree_structure = TreeStructure {
            root_nodes: vec![
                RootNode {
                    label: "Oriënteren".to_owned(),
                    children: parse_structure(boom.orientatiefase.clone().unwrap()),
                },
                RootNode {
                    label: "Plan maken en uitvoeren".to_owned(),
                    children: parse_structure(boom.planfase.clone().unwrap()),
                },
                RootNode {
                    label: "Afronden".to_owned(),
                    children: parse_structure(boom.afrondfase.clone().unwrap()),
                },
            ],
        };

        let isbn = if boom.boek == Some("Abstract Algebra, Dummit & Foote".to_string())
            || boom.boek == Some("abstract Algebra, Dummit & Foote".to_string())
        {
            Some(9_780_471_433_347)
        } else if boom.deel == Some("Havo wiskunde A - H4.wisa".to_string())
            || boom.deel == Some("havo wiskunde a - H4.wisa".to_string())
        {
            Some(9_789_001_892_258)
        } else if boom.deel == Some("vwo wiskunde B deel 1 (klas 4)".to_string()) {
            Some(9_789_001_891_787)
        } else {
            None
        };

        new_trees.push(Tree {
            id: boom.boom_id as i32,
            owner_id: boom.user_id as i32,
            created_at: boom.timestamp,
            last_modified: boom
                .laatst_gewijzigd
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc(),
            published: match boom.gepubliceerd {
                Some(0) => false,
                Some(1) => true,
                _ => unreachable!(),
            },
            times_visited: i64::from(boom.aantal_bezoeken),
            last_visited: boom
                .laatst_bezocht
                .map(|date| date.and_hms_opt(0, 0, 0).unwrap().and_utc()),
            title: recover_string(&boom.titel),
            description: boom
                .omschrijving
                .clone()
                .map(|omschrijving| recover_string(&omschrijving))
                .unwrap_or_default(),
            book_chapter: boom
                .hoofdstuk
                .clone()
                .map(|hoofdstuk| recover_string(&hoofdstuk))
                .unwrap_or_default(),
            book_exercise: boom
                .opgavenummer
                .map(|num| num.to_string())
                .unwrap_or_default(),
            assignment_text: get_converted_opgave(client, boom, api_base_url.clone()).await,
            structure: tree_structure,
            language_code: "nl".to_owned(),
            subject_id: get_subject_id(boom),
            book_isbn: isbn,
            tags: get_tags(boom),
        });
    }

    // Update the next value of the serial id.
    // This is needed because if values are added directly with their id value,
    // the serial counter isn't updated automatically.
    let next_id = new_trees.last().unwrap().id + 1;

    let mut query_builder = QueryBuilder::new(format!(
        "ALTER SEQUENCE trees_id_seq RESTART WITH {next_id}"
    ));
    let postgres_query = query_builder.build();

    postgres_query.execute(postgres_db).await.unwrap();

    // Add the trees to the new database.
    let mut query_builder = QueryBuilder::new(
        "INSERT INTO trees (id, owner_id, created_at, last_modified, published, times_visited, last_visited, title, description, book_chapter, book_exercise, assignment_text, structure, language_code, subject_id, book_isbn) ",
    );
    query_builder.push_values(&new_trees, |mut b, tree| {
        b.push_bind(tree.id)
            .push_bind(tree.owner_id)
            .push_bind(tree.created_at)
            .push_bind(tree.last_modified)
            .push_bind(tree.published)
            .push_bind(tree.times_visited)
            .push_bind(tree.last_visited)
            .push_bind(tree.title.clone())
            .push_bind(tree.description.clone())
            .push_bind(tree.book_chapter.clone())
            .push_bind(tree.book_exercise.clone())
            .push_bind(tree.assignment_text.clone())
            .push_bind(serde_json::to_value(tree.structure.clone()).unwrap())
            .push_bind(tree.language_code.clone())
            .push_bind(tree.subject_id)
            .push_bind(tree.book_isbn);
    });

    let postgres_query = query_builder.build();

    postgres_query.execute(postgres_db).await.unwrap();

    for tree in new_trees {
        add_tags(client, tree, api_base_url.clone()).await;
    }
}

/// Parse the tree structure as stored in the prototype database to the new type used in the database
fn parse_structure(old_format: String) -> Vec<Node> {
    if old_format.is_empty() {
        return vec![];
    }
    let mut old_format = old_format;
    old_format.insert(0, 'a');
    let tree: Vec<&str> = old_format.split('#').collect::<Vec<_>>();

    let mut nodes = vec![];
    let mut i = 0;

    while i != tree.len() {
        let (_, num) = parse_block(tree[i]);

        let (children, new_i) = add_next(&tree, i, 'a');

        nodes.push(Node {
            card_id: num,
            children,
        });
        i = new_i;
    }

    nodes
}

fn add_next(tree: &Vec<&str>, begin: usize, depth: char) -> (Vec<Node>, usize) {
    let mut children = vec![];
    let mut begin = begin;
    loop {
        if begin == tree.len() {
            return (children, begin);
        }

        let (next_depth, num) = parse_block(tree[begin]);

        if next_depth <= depth {
            return (children, begin + 1);
        }
        let (node_children, new_begin) = add_next(tree, begin + 1, next_depth);

        children.push(Node {
            card_id: num,
            children: node_children,
        });
        begin = new_begin;
    }
}

fn parse_block(data: &str) -> (char, i32) {
    let mut chars = data.chars();
    let depth = chars.next().unwrap();
    let num = chars.as_str().parse().unwrap();

    (depth, num)
}

async fn get_converted_opgave(client: &Client, boom: &Boom, api_base_url: String) -> String {
    // Insert image in the assignment if present
    let add_str = if boom.plaatje.clone().unwrap() == "" {
        String::new()
    } else {
        let prototype_path = format!(
            "plaatjes/opgave{}.{}",
            boom.boom_id,
            boom.plaatje.clone().unwrap()
        );
        format!(
            "![{}]({})",
            prototype_path,
            insert_img_in_db(client, &prototype_path, api_base_url.clone()).await
        )
    };

    let raw_text = boom.opgavetekst.clone().unwrap();
    let is_image_fixed: bool;
    let fix_image = if raw_text.contains(r#"<img src=\"plaatjes/opgave505.jpg\">"#) {
        is_image_fixed = true;
        replace_picture_link(client, raw_text, "opgave505.jpg", api_base_url).await
    } else {
        is_image_fixed = false;
        raw_text
    };

    let mut converted_opgave = convert(&recover_string(&fix_image), None)
        .unwrap()
        .content
        .unwrap()
        + add_str.as_str();

    if is_image_fixed {
        converted_opgave = converted_opgave.replace("```", "");
    }

    converted_opgave
}

fn get_subject_id(boom: &Boom) -> Option<i32> {
    if boom
        .boek
        .clone()
        .is_some_and(|boek| boek != "Abstract Algebra, Dummit & Foote")
    {
        Some(1)
    } else if boom.deel.clone().is_some_and(|deel| {
        deel.contains("wiskunde a")
            || deel.contains("wiskunde A")
            || deel.contains("wisa")
            || deel.contains("A Wiskunde")
            || deel.contains("wis A")
    }) {
        Some(2)
    } else if boom
        .deel
        .clone()
        .is_some_and(|deel| deel.contains("wiskunde B") || deel.contains("vwo B"))
    {
        Some(3)
    } else if boom
        .deel
        .clone()
        .is_some_and(|deel| deel.contains("wiskunde D"))
    {
        Some(5)
    } else {
        None
    }
}

/// Extract the book name, leerjaar and schoolsoort from the tree data if possible
fn get_tags(boom: &Boom) -> Vec<String> {
    let mut tags = vec![];

    if let Some(boek) = boom.boek.clone() {
        tags.push(boek);
    }

    let leerjaar = if boom.deel.clone().is_some_and(|deel| {
        deel.contains("1 Havo/vwo")
            || deel.contains("1 havo/vwo")
            || deel.contains("brugklas")
            || deel.contains("klas 1")
            || deel.contains("BK 1")
    }) {
        Some("leerjaar-1".to_string())
    } else if boom.deel.clone().is_some_and(|deel| {
        deel.contains("2 havo/vwo")
            || deel.contains("2 vwo deel1")
            || deel.contains("vwo2")
            || deel.contains("BK 2")
    }) {
        Some("leerjaar-2".to_string())
    } else if boom.deel.clone().is_some_and(|deel| {
        deel.contains("Leerjaar 3") || deel.contains("wiskunde 3") || deel.contains("VMBO3KGT")
    }) {
        Some("leerjaar-3".to_string())
    } else if boom.deel.clone().is_some_and(|deel| deel.contains("4")) {
        Some("leerjaar-4".to_string())
    } else if boom
        .deel
        .clone()
        .is_some_and(|deel| deel.contains("klas 5"))
    {
        Some("leerjaar-5".to_string())
    } else {
        None
    };

    if let Some(leerjaar) = leerjaar {
        tags.push(leerjaar);
    }

    let schoolsoort =
        if boom
            .deel
            .clone()
            .is_some_and(|deel| deel.contains("VMBO Basis"))
        {
            Some("vmbo-bb")
        } else if boom
            .deel
            .clone()
            .is_some_and(|deel| deel.contains("VMBO BK") || deel.contains("VMBO Deel 2 BK"))
        {
            Some("vmbo-kb")
        } else if boom.deel.clone().is_some_and(|deel| {
            deel.contains("KGT")
                || deel.contains("VMBO KGT")
                || deel.contains("vmbo-kgt")
                || deel.contains("VMBO3KGT")
        }) {
            Some("vmbo-kgt")
        } else if boom.deel.clone().is_some_and(|deel| {
            deel.contains("vmbo-t havo")
                || deel.contains("TL/HAVO")
                || deel.contains("t havo")
                || deel.contains("Vmbo-t")
        }) {
            Some("vmbo-t/havo")
        } else if boom
            .deel
            .clone()
            .is_some_and(|deel| deel.contains("havo/vwo") || deel.contains("Havo/vwo"))
        {
            Some("havo/vwo")
        } else if boom.deel.clone().is_some_and(|deel| {
            deel.contains("havo") || deel.contains("Havo") || deel.contains("H4")
        }) {
            Some("havo")
        } else if boom
            .deel
            .clone()
            .is_some_and(|deel| deel.contains("vwo") || deel.contains("VWO"))
        {
            Some("vwo")
        } else {
            None
        };

    if let Some(schoolsoort) = schoolsoort {
        tags.push(schoolsoort.to_string());
    }

    tags
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeData {
    pub title: String,
    pub language_code: String,
    pub tags: Vec<String>,
    pub description: String,
    pub subject_id: Option<i32>,
    pub book_name: String,
    pub book_edition: String,
    pub book_author: String,
    pub book_isbn: Option<String>,
    pub book_chapter: String,
    pub book_exercise: String,
    pub assignment_text: String,
    pub published: bool,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeUpdateData {
    pub id: i32,
    pub new_data: TreeData,
}

async fn add_tags(client: &Client, tree: Tree, api_base_url: String) {
    let response = client
        .post(api_base_url + "/api/v1/trees/update-data")
        .json(&TreeUpdateData {
            id: tree.id,
            new_data: TreeData {
                title: tree.title,
                language_code: tree.language_code,
                tags: tree.tags,
                description: tree.description,
                subject_id: tree.subject_id,
                book_name: String::new(),
                book_edition: String::new(),
                book_author: String::new(),
                book_isbn: tree.book_isbn.map(|isbn| isbn.to_string()),
                book_chapter: tree.book_chapter,
                book_exercise: tree.book_exercise,
                assignment_text: tree.assignment_text,
                published: tree.published,
            },
        })
        .send()
        .await
        .unwrap();

    println!(
        "status: {}. text: {}",
        response.status().as_str(),
        response.text().await.unwrap()
    );
}
