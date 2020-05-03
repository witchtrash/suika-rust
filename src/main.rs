mod jobs;

fn main() {
    println!("Hello, world!");
    println!("{:#?}", jobs::scrape::get());
}
