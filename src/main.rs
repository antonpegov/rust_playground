mod config;
mod models;
mod routes;

use crate::config::print_config as log_config;
use crate::routes::user_route;

fn main() {
    println!("Hello, world!");

    log_config();
    user_route::print_user_route();
    routes::index_route::print_index_route();
}
