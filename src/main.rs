#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket::State;
use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use std::collections::HashSet;

struct Data {
    egn: HashSet<String>,
    emails: HashSet<String>,
}

#[derive(FromForm)]
struct SearchForm {
    egn: String,
    email: String
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("egn", "");
    context.insert("email", "");
    Template::render("index", context)
}

#[post("/", data = "<search>")]
fn search(data: State<Data>, search: Form<SearchForm>) -> Template {
    let mut context = Context::new();
    context.insert("egn", &search.egn);
    context.insert("email", &search.email);

    if !search.egn.is_empty() {
        context.insert("egn_found", &data.egn.contains(&search.egn));
    }

    if !search.email.is_empty() {
        context.insert("email_found", &data.emails.contains(&search.email));
    }

    Template::render("index", context)
}

fn main() {
    if let Ok(egn) = std::fs::read_to_string("resources/egn.txt") {
        if let Ok(emails) = std::fs::read_to_string("resources/emails.txt") {
            let egn_list: HashSet<String> = egn.split("\n").map(String::from).into_iter().collect();
            let email_list: HashSet<String> = emails.split("\n").map(String::from).into_iter().collect();

            let config = Data {
                egn: egn_list,
                emails: email_list,
            };

            rocket::ignite()
                .manage(config)
                .mount("/", routes![index, search])
                .mount("/assets", StaticFiles::from("assets"))
                .attach(Template::fairing())
                .launch();
        }
    }
}
