/*****************************************************************
* SQLite3 connection pool sample
* https://github.com/forcia/rustbook/tree/master/ch05/5-3
******************************************************************/
use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use thiserror::Error;

const SERVER_URL: &str = "127.0.0.1:8080";
const DB_NAME: &str = "todo.db";

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

    HttpServer::new(move || App::new().service(index).data(pool.clone()))
        .bind(SERVER_URL)?
        .run()
        .await?;
    Ok(())
}
