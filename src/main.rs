mod api_handler;
mod github_client;
mod path_parser;
mod directory;
mod file;

use api_handler::api_handler;
use warp::Filter;

#[tokio::main]
async fn main() {
    let api = warp::any().and(warp::path::full()).and_then(api_handler);

    let welcome = warp::path::end().map(|| format!("Welcome to my api"));

    let routes = welcome.or(api);
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}