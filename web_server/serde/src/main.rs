/*****************************************************************
* serde POST request parse sample
* https://github.com/forcia/rustbook/blob/master/ch05/5-4
******************************************************************/
use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;
use thiserror::Error;

const SERVER_URL: &str = "127.0.0.1:8080";
const DB_NAME: &str = "todo.db";

// serde macro for deserialize
#[derive(Deserialize)]
struct AddParams {
    text: String,
}

// serde macro for deserialize
#[derive(Deserialize)]
struct DeleteParams {
    id: u32,
}

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

// wrap 3 kinds of errors
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConncectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}

// actix-web POST
#[post("/add")]
async fn add_todo(
    params: web::Form<AddParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?;

    // HttpResponse::SeeOther(): HTTP STATUS CODE 303
    // redirect to "/"
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

// actix-web POST
#[post("/delete")]
async fn delete_todo(
    params: web::Form<DeleteParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("DELETE FROM todo WHERE id=?", &[&params.id])?;

    // HttpResponse::SeeOther(): HTTP STATUS CODE 303
    // redirect to "/"
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id, text })
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }
    // Init askama::Template struct with Vec.
    let html = IndexTemplate { entries };
    // render html
    let response_body = html.render()?;
    // HttpResponse::Ok(): HTTP STATUS CODE 200
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    //sqlite3 connection pool
    let manager = SqliteConnectionManager::file(DB_NAME);
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    )
    .expect("Failed to create a table `todo`.");

    conn.execute(
        "INSERT INTO todo VALUES(1, \"Initial first entry\")",
        params![],
    )
    .expect("Failed to insert into `todo`.");

    conn.execute(
        "INSERT INTO todo VALUES(2, \"Initial second entry\")",
        params![],
    )
    .expect("Failed to insert into `todo`.");

    println!("HTTP Server http://{} starting...", SERVER_URL);

    // add "add_todo", "delete_todo" entries.
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add_todo)
            .service(delete_todo)
            .data(pool.clone())
    })
    .bind(SERVER_URL)?
    .run()
    .await?;
    Ok(())
}
