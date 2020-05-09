#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod suika;

fn main() {
    // println!("{:#?}", suika::job::scrape::get());
    rocket::ignite()
        .mount("/", routes![suika::api::index])
        .launch();
}
