/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

mod books;
mod hints;
mod trees;
mod users;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions};

use crate::{
    books::create_books, hints::migrate_hints, trees::migrate_trees, users::migrate_users,
};

use std::env;

/// The body for a login request that contains the email address and password.
#[derive(Deserialize, Serialize)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}

#[tokio::main]
async fn main() {
    println!("Start");

    // Connect to databases, using environment variables
    if let Err(err) = dotenvy::dotenv() {
        assert!(
            err.not_found(),
            "The .env file was found but could not be loaded: {err}"
        );
    }

    let postgres_url =
        env::var("POSTGRES_URL").expect("Environment variable POSTGRES_URL must be set");

    let postgres_db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&postgres_url)
        .await
        .expect("failed to connect to postgres database.");

    let mysql_url =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set");

    let mysql_db = MySqlPoolOptions::new()
        .connect(&mysql_url)
        .await
        .expect("failed to connect to mysql database.");

    sqlx::migrate!()
        .run(&mysql_db)
        .await
        .expect("failed to run prototype database migration");

    println!("Connected to databases");

    let migrator_email =
        env::var("MIGRATOR_EMAIL").expect("Environment variable MIGRATOR_EMAIL must be set");
    let migrator_password =
        env::var("MIGRATOR_PASSWORD").expect("Environment variable MIGRATOR_PASSWORD must be set");

    migrate_users(&mysql_db, &postgres_db, &migrator_email).await;

    println!("Migrated users");

    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap();

    println!("Created client");

    let api_base_url =
        env::var("API_BASE_URL").expect("Environment variable API_BASE_URL must be set");

    let response = client
        .post(api_base_url.clone() + "/api/v1/auth/login")
        .json(&LoginRequestBody {
            email: migrator_email,
            password: migrator_password,
        })
        .send()
        .await
        .unwrap();

    println!("Logged in: {}", response.text().await.unwrap());

    create_books(&postgres_db).await;

    println!("Created books");

    migrate_trees(&mysql_db, &postgres_db, &client, api_base_url.clone()).await;

    println!("Migrated trees");

    migrate_hints(&mysql_db, &postgres_db, &client, api_base_url).await;

    println!("Migrated hints");
    println!("Succeeded!");
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
async fn insert_img_in_db(
    client: &reqwest::Client,
    relative_path: &str,
    api_base_url: String,
) -> String {
    let full_path = "https://edspace.nl/hboom/".to_owned() + relative_path;

    let image = reqwest::get(&full_path)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    println!("Got image {relative_path}");

    client
        .post(api_base_url + "/api/v1/images/add")
        .body(image)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

async fn replace_picture_link(
    client: &Client,
    content: String,
    short_link: &str,
    api_base_url: String,
) -> String {
    content.replace(
        "src=",
        &format!(
            "src={} alt=plaatjes/{}",
            insert_img_in_db(client, &format!("plaatjes/{short_link}"), api_base_url).await,
            short_link
        ),
    )
}
