mod db;
mod io_utils;
mod models;
mod ui;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    println!("Welcome To My-Jira!",);
}
