#![feature(proc_macro_hygiene, decl_macro)]

mod suika;

use diesel;
use rocket;

use rocket::{routes};
use suika::api;

fn main() {
    // println!("{:#?}", suika::job::scrape::get());
    rocket::ignite()
        .mount("/", routes![api::index, api::scrape])
        .launch();
}
