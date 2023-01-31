use reqwest::{header::{USER_AGENT, ACCEPT}, Url};
use uuid::Uuid;
use std::{path::{Path, PathBuf}, fs, io};
use serde::Deserialize;

use crate::{path_parser::RequestMetaData, directory::{Directory, File}};

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

// needs tests
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

pub async fn get_file_from_github(file: &File) {
    println!("Downloading file from: {:?}", file.url.as_str());
    let client = reqwest::Client::new();
    match client.get(file.url.as_str())
            .header(ACCEPT, "application/vnd.github.v3.raw")
            .send()
            .await {
        Ok(response) => {
            let mut file = fs::File::create(&file.path).unwrap();
            let mut content =  io::Cursor::new(response.bytes().await.unwrap());
            io::copy(&mut content, &mut file).unwrap();
        },
        Err(e) => {
            println!("Couldn't download because of {:?}", e);
        }
    }
}

fn generate_uuid() -> String {
    return Uuid::new_v4().to_string();
}
#[derive(Debug)]
pub struct GithubData {
    pub id: String,
    pub path: PathBuf,
    // pub result: Directory,
    pub res_path: PathBuf,
}

impl GithubData {
    //  pub async fn new(response: &Vec<ResponseObject>) -> Self {
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
        // result_dir.update_from_github_api().await;
        println!("Updated from github for top level result directory");
        (GithubData {
            path: work_path,
            res_path: Path::new(".").join("tmp").join(format!("{}.tar.gz",&uuid)),
            id: uuid,
            // result: result_dir
        }, result_dir)
    }

    // // fn download() -> Result<std::path::Path, >
    // pub async fn download_and_zip(&self) {
    //     self.result.update_from_github_api().await;
    //     println!("Starting download");
    //     self.result.download_from_github().await;
    //     println!("Download complete");
        
    //     println!("Starting zip at position: {:?}", self.res_path);
    //     let tar_gz = fs::File::create(&self.res_path).unwrap();
    //     let enc = GzEncoder::new(tar_gz, Compression::default());
    //     let mut tar = tar::Builder::new(enc);
    //     tar.append_dir_all(&self.id, &self.path).unwrap();
    //     println!("Zip complete");
    // }
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