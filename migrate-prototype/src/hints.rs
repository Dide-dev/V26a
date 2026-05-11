use std::io::Write;

use html_to_markdown_rs::convert;
use reqwest::Client;
use sqlx::{MySqlExecutor, PgExecutor, QueryBuilder, query_as};

use crate::{insert_img_in_db, recover_string, replace_picture_link};

#[derive(Debug)]
pub struct Hint {
    pub hint_id: u32,
    pub boom_id: u32,
    pub aanwijzing: Option<String>,
    pub hinttekst: Option<String>,
    pub plaatje: Option<String>,
}

#[derive(Debug)]
pub struct CardData {
    pub id: i32,
    pub tree_id: i32,
    pub front: String,
    pub back: String,
}

pub async fn migrate_hints(
    mysql_db: impl MySqlExecutor<'_>,
    postgres_db: impl PgExecutor<'_>,
    client: &Client,
) {
    let hints = query_as!(Hint, "SELECT * FROM `hints`")
        .fetch_all(mysql_db)
        .await
        .unwrap();

    // let mut image_paths = vec![];

    let mut new_hints = vec![];

    for hint in &hints {
        let add_str = if hint.plaatje.clone().unwrap() == "" {
            String::new()
        } else {
            let prototype_path = format!(
                "plaatjes/hint{}h{}.{}",
                hint.boom_id,
                hint.hint_id,
                hint.plaatje.clone().unwrap()
            );
            format!(
                "![{}]({})",
                prototype_path,
                insert_img_in_db(client, &prototype_path).await
            )
        };

        let raw_back = hint.hinttekst.clone().unwrap_or_default();
        let is_image_fixed: bool;

        let fix_images = if raw_back.contains(r#"<img src=\"plaatjes/hint505h7142.jpg\">"#) {
            is_image_fixed = true;
            replace_picture_link(client, raw_back, "hint505h7142").await
        } else if raw_back.contains(r#"<img src=\"plaatjes/hint505h7147.jpg\">"#) {
            is_image_fixed = true;
            replace_picture_link(client, raw_back, "hint505h7147").await
        } else {
            is_image_fixed = false;
            raw_back
        };

        let mut back = convert(&recover_string(&fix_images), None)
            .unwrap()
            .content
            .unwrap()
            + add_str.as_str();

        if is_image_fixed {
            back = back.replace("```", "");
        }

        new_hints.push(CardData {
            id: hint.hint_id as i32,
            tree_id: hint.boom_id as i32,
            front: recover_string(&hint.aanwijzing.clone().unwrap_or_default()),
            back,
        });
    }

    // let file_name = "new_hints_md.txt";
    // let _file = std::fs::File::create(file_name);

    // let mut data_file = std::fs::OpenOptions::new()
    //     .append(true)
    //     .open(file_name)
    //     .expect("Failed to open data file");
    // data_file
    //     .write_all(format!("{new_hints:#?}").as_bytes())
    //     .expect("Failed to write to data file");

    let mut query_builder = QueryBuilder::new("INSERT INTO cards (id, tree_id, front, back) ");
    query_builder.push_values(new_hints, |mut b, hint| {
        b.push_bind(hint.id)
            .push_bind(hint.tree_id)
            .push_bind(hint.front)
            .push_bind(hint.back);
    });

    let postgres_query = query_builder.build();

    postgres_query.execute(postgres_db).await.unwrap();
}
