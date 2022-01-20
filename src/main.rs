use actix_files::Files as ActixFiles;
use actix_web::web::{route, Form};
use actix_web::{get, post, App, HttpRequest, HttpServer, Responder, Result as ActixResult};
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

mod page;
mod strings;

#[get("/")]
async fn index(req: HttpRequest) -> ActixResult<Markup> {
    let host = format!("{}", req.uri());
    let title = "actix-maud-htmx-h5bp";
    let desc = "This is a template. There are many like it but this one is mine.";
    let lang = "en";
    // TODO: Add your site or application content here.
    let content = html! {
        #content {
            p { "Hello world! This is HTML5 Boilerplate." }
        }
        form hx-post="/hello" hx-target="#content" hx-swap="outerHTML" {
            div {
                label { "What's your name? " }
                input type="text" name="name" value="" {}
            }
            button { "Submit" }
        }
    };
    Ok(page::page(&host, title, desc, lang, content))
}

#[post("/hello")]
async fn hello(user_input: Form<HelloForm>) -> ActixResult<Markup> {
    Ok(html! {
        #content {
            p { "Hello " (user_input.name) "! This is HTMX." }
        }
    })
}

async fn not_found() -> impl Responder {
    (
        html! {
            html lang="en" {
                head {
                    meta charset=(strings::UTF8);
                    title { (strings::NOT_FOUND_TITLE) }
                    meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
                    style { (strings::NOT_FOUND_STYLE) }
                }
                body {
                    h1 { (strings::NOT_FOUND_TITLE) }
                    p { (strings::NOT_FOUND_MESSAGE) }
                }
                (PreEscaped(strings::NOT_FOUND_COMMENT))
            }
        },
        actix_web::http::StatusCode::NOT_FOUND,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
            .default_service(route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Deserialize)]
struct HelloForm {
    name: String,
}
