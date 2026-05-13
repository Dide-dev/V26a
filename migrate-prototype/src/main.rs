#![warn(clippy::pedantic)]

mod hints;
mod trees;
mod users;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions};

use crate::{hints::migrate_hints, trees::migrate_trees, users::migrate_users};

// use std::env;

/// The body for a login request that contains the email address and password.
#[derive(Deserialize, Serialize)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}

#[tokio::main]
async fn main() {
    println!("Start");

    let database_url = "postgresql://user:password@localhost/postgres_db";
    // env::var("POSTGRES_URL").expect("Environment variable DATABASE_URL must be set");

    let postgres_db = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .expect("failed to connect to postgres database.");

    let mysql_db = MySqlPoolOptions::new()
        .connect("mysql://root:password@localhost:3307/mysql_db")
        .await
        .expect("failed to connect to mysql database.");

    sqlx::migrate!()
        .run(&mysql_db)
        .await
        .expect("failed to run prototype database migration");

    println!("Connected to databases");

    migrate_users(&mysql_db, &postgres_db).await;

    println!("Migrated users");

    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap();

    println!("Created client");

    let response = client
        // .post("https://boom.its.uu.nl/api/v1/auth/login")
        .post("http://localhost:3000/api/v1/auth/login")
        .json(&LoginRequestBody {
            email: "j.quist1@students.uu.nl".to_owned(),
            password: "heuristics!".to_owned(),
        })
        .send()
        .await
        .unwrap();

    println!("Logged in: {}", response.text().await.unwrap());

    migrate_trees(&mysql_db, &postgres_db, &client).await;
    migrate_hints(&mysql_db, &postgres_db, &client).await;
}

fn recover_string(old_str: &str) -> String {
    // Thanks to https://www.i18nqa.com/debug/utf8-debug.html
    old_str
        .replace("Ã–", "Ö")
        .replace("Ã¼", "ü")
        .replace("Ã§", "ç")
        .replace("Ã©", "é")
        .replace("Ã«", "ë")
        .replace("ðŸ˜‰", "😉")
        .replace("Ã¯", "ï")
        .replace("Ã—", "×")
        .replace(r"\\", r"\")
        .replace("&#39;", "'")
        .replace("&#34;", "\"")
        .replace("â€“", "–")
        .replace("â€œ", "“")
        .replace("â€", "”")
        .replace("â€¢", "•")
        .replace("É¸", "ɸ")
        // Update links to old images
        .replace(r#"src=\""#, "src=")
        .replace(r#"\" alt"#, " alt")
}

// Returns the relative permanent url of the image
async fn insert_img_in_db(client: &reqwest::Client, relative_path: &str) -> String {
    let full_path = "https://edspace.nl/hboom/".to_owned() + relative_path;

    let image = reqwest::get(&full_path)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    println!("Got image {relative_path}");

    client
        // .post("https://boom.its.uu.nl/api/v1/images/add")
        .post("http://localhost:3000/api/v1/images/add")
        .body(image)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

async fn replace_picture_link(client: &Client, content: String, short_link: &str) -> String {
    content.replace(
        "src=",
        &format!(
            "src={} alt=plaatjes/{}",
            insert_img_in_db(client, &format!("plaatjes/{short_link}")).await,
            short_link
        ),
    )
}
