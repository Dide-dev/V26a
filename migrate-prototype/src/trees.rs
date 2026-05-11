use std::io::Write;

use html_to_markdown_rs::{
    ConversionOptions, ConversionOptionsBuilder, PreprocessingOptions, convert,
};

use reqwest::Client;
use sqlx::{
    MySqlExecutor, PgExecutor, PgPool, QueryBuilder, query_as,
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
    editie: Option<String>,
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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeStructure {
    pub root_nodes: Vec<RootNode>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RootNode {
    pub label: String,
    pub children: Vec<Node>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
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
    pub description: Option<String>,
    pub book_name: Option<String>,
    pub book_edition: Option<String>,
    pub book_isbn: Option<String>,
    pub book_chapter: Option<String>,
    pub book_exercise: Option<String>,
    pub assignment_text: Option<String>,
    pub tree_structure: TreeStructure,
}

pub async fn migrate_trees(
    mysql_db: impl MySqlExecutor<'_>,
    postgres_db: impl PgExecutor<'_>,
    // postgres_db: PgPool,
    client: &Client,
) {
    let bomen = query_as!(
        Boom,
        "SELECT boom_id, timestamp, user_id, laatst_gewijzigd, gepubliceerd, 
        aantal_bezoeken as `aantal_bezoeken: u32`, laatst_bezocht, titel, boek, 
        editie, deel, hoofdstuk, opgavenummer, omschrijving, opgavetekst, plaatje, 
        orientatiefase, planfase, afrondfase
        FROM `bomen`"
    )
    .fetch_all(mysql_db)
    .await
    .unwrap();

    let mut new_trees = vec![];

    for boom in &bomen {
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
                insert_img_in_db(client, &prototype_path).await
            )
        };

        let raw_text = boom.opgavetekst.clone().unwrap();
        let is_image_fixed: bool;
        let fix_image = if raw_text.contains(r#"<img src=\"plaatjes/opgave505.jpg\">"#) {
            is_image_fixed = true;
            replace_picture_link(client, raw_text, "opgave505.jpg").await
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
            converted_opgave = converted_opgave.replace("```", "")
        }

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

        new_trees.push(Tree {
            id: boom.boom_id as i32,
            owner_id: boom.user_id as i32,
            created_at: boom
                .laatst_gewijzigd
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc(),
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
                .map(|omschrijving| recover_string(&omschrijving)),
            book_name: boom.boek.clone().map(|boek| recover_string(&boek)),
            book_edition: boom.editie.clone().map(|editie| recover_string(&editie)),
            book_isbn: None,
            book_chapter: boom
                .hoofdstuk
                .clone()
                .map(|hoofdstuk| recover_string(&hoofdstuk)),
            book_exercise: boom.opgavenummer.map(|num| num.to_string()),
            assignment_text: Some(converted_opgave),
            tree_structure,
        });
    }

    // let file_name = "new_trees.txt";
    // let _file = std::fs::File::create(file_name);

    // let mut data_file = std::fs::OpenOptions::new()
    //     .append(true)
    //     .open(file_name)
    //     .expect("Failed to open data file");
    // data_file
    //     .write_all(format!("{new_trees:#?}").as_bytes())
    //     .expect("Failed to write to data file");

    let mut query_builder = QueryBuilder::new(
        "INSERT INTO trees (id, owner_id, created_at, last_modified, published, times_visited, last_visited, title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, structure) ",
    );
    query_builder.push_values(new_trees, |mut b, tree| {
        b.push_bind(tree.id)
            .push_bind(tree.owner_id)
            .push_bind(tree.created_at)
            .push_bind(tree.last_modified)
            .push_bind(tree.published)
            .push_bind(tree.times_visited)
            .push_bind(tree.last_visited)
            .push_bind(tree.title)
            .push_bind(tree.description)
            .push_bind(tree.book_name)
            .push_bind(tree.book_edition)
            .push_bind(tree.book_isbn)
            .push_bind(tree.book_chapter)
            .push_bind(tree.book_exercise)
            .push_bind(tree.assignment_text)
            .push_bind(serde_json::to_value(tree.tree_structure).unwrap());
    });

    let postgres_query = query_builder.build();

    postgres_query.execute(postgres_db).await.unwrap();

    // for tree in new_trees {
    //     let mut query_builder = QueryBuilder::new("UPDATE trees SET structure = ");

    //     query_builder
    //         .push_bind(serde_json::to_value(tree.tree_structure).unwrap())
    //         .push(" WHERE id = ")
    //         .push_bind(tree.id);

    //     let postgres_query = query_builder.build();

    //     postgres_query.execute(&postgres_db).await.unwrap();
    // }
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
