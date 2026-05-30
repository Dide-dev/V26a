/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use sqlx::{PgPool, QueryBuilder};

struct Book {
    isbn: i64,
    title: String,
    author: String,
    edition: String,
}

pub async fn create_books(postgres_db: &PgPool) {
    let books = vec![
        Book {
            isbn: 9_789_001_891_787,
            title: "Getal & Ruimte vwo B leerboek 1".to_owned(),
            author: "Noordhoff".to_owned(),
            edition: "12.0".to_owned(),
        },
        Book {
            isbn: 9_789_001_892_258,
            title: "Moderne Wiskunde havo A leerboek 1".to_owned(),
            author: "Noordhoff".to_owned(),
            edition: "12.0".to_owned(),
        },
        Book {
            isbn: 9_780_471_433_347,
            title: "Abstract Algebra".to_owned(),
            author: "Dummit & Foote".to_owned(),
            edition: "3".to_owned(),
        },
    ];

    let mut query_builder = QueryBuilder::new("INSERT INTO books (isbn, title, author, edition) ");

    query_builder.push_values(books, |mut b, book| {
        b.push_bind(book.isbn)
            .push_bind(book.title)
            .push_bind(book.author)
            .push_bind(book.edition);
    });

    let postgres_query = query_builder.build();

    postgres_query.execute(postgres_db).await.unwrap();
}
