use reqwest::{header::USER_AGENT, Url};
use uuid::Uuid;
use std::path::{Path, PathBuf};
use serde::Deserialize;

use crate::{path_parser::RequestMetaData, directory::Directory};

#[derive(Debug, Deserialize)]
pub struct ResponseObject {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub download_url: String,
    pub r#type: String,
    pub size: u32
}

/* todo: add tests for utilities */ 
pub async fn get_from_github_api(valid_url: &Url) -> Vec<ResponseObject>{
    println!("Getting from github api: {:?}", valid_url.as_str());
    let client = reqwest::Client::new();
    client.get(valid_url.as_str()) // hacky but works for now
        .header(USER_AGENT, "nsrcodes")
        .send()
        .await
        .unwrap()
        .json::<Vec<ResponseObject> >()
        .await
        .unwrap()
}

fn generate_uuid() -> String {
    return Uuid::new_v4().to_string();
}
#[derive(Debug)]
pub struct GithubData {
    pub id: String,
    pub path: PathBuf,
    pub res_path: PathBuf,
}

impl GithubData {
    pub fn new(request: RequestMetaData) -> (Self, Directory) {
        let uuid = generate_uuid();
        let mut work_path = Path::new(".").join("tmp");
        work_path.push(&uuid);

        let name = match request.path.as_str().rsplit_once("/") {
            Some((_, name)) => name,
            None => request.path.as_str(),
        };

        let path = work_path.join(&name);

        let result_dir = Directory::new(name.to_string(), path, request.api_target());
        println!("Updated from github for top level result directory");
        (GithubData {
            path: work_path,
            res_path: Path::new(".").join("tmp").join(format!("{}.tar.gz",&uuid)),
            id: uuid,
            // result: result_dir
        }, result_dir)
    }
}

#[cfg(test)]
mod github_client_tests {
    // write a dummy Github data struct creator

    #[test]
    fn deserielize_github_response() {
        // todo
        // define the prototype of this function
        // give the function an example github response
        // compare it with the expected response struct object
    }

    #[test]
    fn test_download_single_file() {
        // todo
        // create dummy object for one single file (no directories)
        // call download
        // check if a result folder was created
        // verifu the contents of the result folder
    }

    #[test]
    fn test_download_nested_directory() {
        // todo
        // create dummy object for nested files and directories)
        // call download
        // check if a result folder was created
        // verify the contents of the result folder,
        // Should match hierarchy of the dummy nested structures
    }
}