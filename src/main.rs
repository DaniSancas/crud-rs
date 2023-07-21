mod application;
mod domain;
mod infrastructure;

use infrastructure::article_controller;

fn main() {
    println!("{:?}", article_controller::get_article(1));
}
