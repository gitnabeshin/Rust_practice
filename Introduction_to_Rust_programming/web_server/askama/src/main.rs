use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

const SERVER_URL: &str = "127.0.0.1:8080";

struct TodoEntry {
    id: u32,
    text: String,
}

// askama macro
// https://github.com/djc/askama
#[derive(Template)]               // askamaでコードを生成するおまじない
#[template(path = "index.html")]  // "templates" ディレクトリのファイルを指定
struct IndexTemplate {
    entries: Vec<TodoEntry>,      // 変数名はtamplateの名称と合わせる
}

// thiserror macro
// https://github.com/dtolnay/thiserror
// 異なるエラー型を持たせる
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    InAskamaError(#[from] askama::Error),
}

// 独自のエラー構造体を作成
// traitを使ってポリモーフィズムを実現
// 異なる型のエラーを使えるようにResponseErrorを拡張する
impl ResponseError for MyError {}

#[get("/")]
// async fn index() -> Result<HttpResponse, actix_web::Error> {
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });
    let html = IndexTemplate { entries };
    let response_body = html.render()?;    // render html from template and values
    // dbg!("{}", &response_body);
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind(SERVER_URL)?
        .run()
        .await?;
    Ok(())
}
