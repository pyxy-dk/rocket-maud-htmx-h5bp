#[macro_use]
extern crate rocket;

use maud::{html, Markup, PreEscaped};
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::request::{FromRequest, Outcome, Request};

mod page;
mod strings;

#[get("/")]
fn index(host: HostHeader) -> Markup {
    let title = "rocket-maud-h5bp";
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
    page::page(host, title, desc, lang, content)
}

#[post("/hello", data = "<user_input>")]
fn hello(user_input: Form<HelloForm>) -> Markup {
    html! {
        #content {
            p { "Hello " (user_input.name) "! This is HTMX." }
        }
    }
}

#[catch(404)]
fn not_found() -> Markup {
    html!(
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
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![hello, index])
        .mount("/", FileServer::from("src/static"))
}

#[derive(FromForm)]
struct HelloForm<'r> {
    name: &'r str,
}

struct HostHeader<'r>(pub &'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostHeader<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Host") {
            Some(h) => Outcome::Success(HostHeader(h)),
            None => Outcome::Forward(()),
        }
    }
}
