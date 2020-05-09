use rocket::{get};
use crate::suika::job::scrape;

#[get("/")]
pub fn index() -> &'static str {
    "Suika v0.1"
}

#[get("/scrape")]
pub fn scrape() -> String {
    format!("{:#?}", scrape::get())
}
