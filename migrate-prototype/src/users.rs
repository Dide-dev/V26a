/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{MySqlExecutor, PgPool, QueryBuilder, query_as};

use crate::recover_string;

#[derive(Debug)]
pub struct User {
    pub user_id: u32,

    pub created_at: DateTime<Utc>,
    pub email: String,
    pub pwd_hash: String,
    pub naam: String,
    pub pseudoniem: Option<String>,
    pub persoonsbeschrijving: Option<String>,
    pub emailvermelding: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug)]
pub enum UserLanguage {
    Nl,
    En,
}

impl Display for UserLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            UserLanguage::Nl => write!(f, "nl"),
            UserLanguage::En => write!(f, "en"),
        }
    }
}

impl FromStr for UserLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "nl" => Ok(UserLanguage::Nl),
            "en" => Ok(UserLanguage::En),
            _ => Err("Een taal is niet gelijk aan nl of en".to_owned()),
        }
    }
}

#[derive(Debug)]
pub struct NewUser {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub nickname: String,
    pub about_me: String,
    pub created_at: DateTime<Utc>,
    pub email_visible: bool,
    pub account_type: AccountType,
    pub language_code: UserLanguage,
}

#[derive(Debug)]
pub enum AccountType {
    Admin,
    Author,
}

impl Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AccountType::Admin => write!(f, "admin"),
            AccountType::Author => write!(f, "author"),
        }
    }
}

pub async fn migrate_users(mysql_db: impl MySqlExecutor<'_>, postgres_db: &PgPool) {
    let users = 
        query_as!(User, 
            "SELECT user_id, created_at, email, pwd_hash, naam, pseudoniem, persoonsbeschrijving, emailvermelding, language 
            FROM `users`")
        .fetch_all(mysql_db)
        .await.unwrap();

    let mut user_names = HashSet::new();

    let new_users = users.iter().map(|user| NewUser {
        id: user.user_id as i32,
        email: user.email.clone(),
        password_hash: user.pwd_hash.clone(),
        name: {
            let polished = polish_user_name(&recover_string(&user.naam));
            if user_names.contains(&(polished.clone() + "1")) {
                user_names.insert(polished.clone() + "2");
                polished + "2"
            } else if user_names.contains(&polished) {
                user_names.insert(polished.clone() + "1");
                polished + "1"
            } else {
                user_names.insert(polished.clone());
                polished
            }
        },
        nickname: match user.pseudoniem.clone().unwrap_or_default() {
            s if s.is_empty() => recover_string(&user.naam),
            s => s
        },
        about_me: user.persoonsbeschrijving.clone().unwrap_or_default(),
        created_at: user.created_at,
        email_visible: match user.emailvermelding.clone().unwrap_or_default().as_str() {
            "0" => false,
            "1" => true,
            _ => panic!("email visible not set in prototype database. User: {user:?}")
        },
        account_type: match user.email.as_str() {
            "r.d.bos@uu.nl" | "theo.vandenbogaart@hu.nl" | "j.quist1@students.uu.nl" => AccountType::Admin,
            _ => AccountType::Author
        },
        language_code: UserLanguage::from_str(&user.language.clone().unwrap_or_default()).unwrap()
    }).collect::<Vec<_>>();

    // Update the next value of the serial id.
    let next_id = new_users.last().unwrap().id + 1;

    let mut query_builder = QueryBuilder::new(format!(
        "ALTER SEQUENCE users_id_seq RESTART WITH {next_id}"
    ));
    let postgres_query = query_builder.build();

    postgres_query.execute(postgres_db).await.unwrap();

    // Add the users to the new database.
    let mut query_builder = QueryBuilder::new("INSERT INTO users (id, email, password_hash, name, nickname, about_me, created_at, email_visible, account_type, language_code) ");
    query_builder.push_values(new_users, |mut b, user| 
            {
                b.push_bind(user.id)
                 .push_bind(user.email)
                 .push_bind(user.password_hash)
                 .push_bind(user.name)
                 .push_bind(user.nickname)
                 .push_bind(user.about_me)
                 .push_bind(user.created_at)
                 .push_bind(user.email_visible)
                 .push_bind(user.account_type.to_string())
                 .push_bind(user.language_code.to_string());
            });
    
    let postgres_query = query_builder.build();

    postgres_query.execute(postgres_db).await.unwrap();
    
}

fn polish_user_name(old_str: &str) -> String {
    old_str.replace("Ö", "O")
           .replace("ü", "u")
           .replace("ç", "c")
           .replace("é", "e")
           .replace("ë", "e")
           .replace(" ", "_")
           .replace("'", "")
}