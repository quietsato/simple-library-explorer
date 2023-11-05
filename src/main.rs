mod api;
mod controllers;
mod models;

use api::reserve;
use controllers::{check, list_libraries};
use env_logger;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // check().unwrap();

    dbg!(reserve().unwrap());
}
