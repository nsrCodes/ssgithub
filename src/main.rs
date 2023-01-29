mod api_handler;
mod github_client;
mod path_parser;
mod directory;

use api_handler::api_handler;
use warp::Filter;

#[tokio::main]
async fn main() {
    let api = warp::any().and(warp::path::full()).and_then(api_handler);

    let welcome = warp::path::end().map(|| format!("Welcome to my api"));

    let routes = welcome.or(api);
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

#[cfg(test)]
mod tests {
    /*
        both failing and passing tests for:
        parsing a path and
            converting to metadata struct
            throwing error if not valid array
            removing the master branch ref

        getting api result using the metadata
            converting to data struct
            case when number of files is greater than 1000 in the directory // should just fail (because it would require too many backend requests)
            case when fetching nested directories

        verifying downloaded results
            verify that it points to file if the path contains a file
            verify if tar was created if the path points to folder
            verify the contents of the result with the contents in data struct
            cleanup function to delete created download asset
    */

    // #[test]
    // fn test_path_to_metadata() {
    //     let path = FullPath::from("/some/ai");
    //     let testObj =  PathMetaData::new(path);
    // }
}
