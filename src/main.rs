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

    let welcome = warp::path::end().map(|| format!("Home page is currently under construction....🛠️"));

    let routes = welcome.or(api);
    println!("starting server on port 8888");
    warp::serve(routes).run(([0, 0, 0, 0], 8888)).await;
}