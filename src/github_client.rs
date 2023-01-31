use reqwest::{header::USER_AGENT, Url};
use tokio::{fs, join};
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
        (GithubData {
            path: work_path,
            res_path: Path::new(".").join("tmp").join(format!("{}.tar.gz",&uuid)),
            id: uuid,
        }, result_dir)
    }

    pub async fn clean(self) {
        let _results = join!(
            fs::remove_dir_all(&self.path),
            fs::remove_file(&self.res_path)
        );
        println!("resses: {:#?}", _results);
    }
}

#[cfg(test)]
mod github_client_tests {
    // write a dummy Github data struct creator

    #[test]
    fn deserielize_github_response() {
        // todo
        // give the function an example github response
        // compare it with the expected response struct object
    }
}