use flate2::{Compression, write::GzEncoder};
use reqwest::{Url, header::ACCEPT};
use std::{
    fs,
    io,
    path::PathBuf, 
};
use async_recursion::async_recursion;

use crate::github_client::get_from_github_api;

#[derive(Debug)]
struct File {
    _name: String,
    path: PathBuf,
    url: Url,
    _hash: String,
    _size: u32,
}
#[derive(Debug)]
pub struct Directory {
    name: String,
    path: PathBuf,
    files: Vec<File>,
    dirs: Vec<Directory>,
    url: Url,
    // hash: String,
    // size: u32,
}
impl Directory {
    pub fn new(name: String, path: PathBuf, url: Url) -> Self {
        Directory { 
            name, 
            path, 
            url,
            files: vec![], 
            dirs: vec![], 
            // hash: String::from(""),
            // size: 0 
        }
    }
    #[async_recursion]
    pub async fn update_from_github_api(&mut self) {
        let mut dir_details_response = get_from_github_api(&self.url).await;

        while let Some(content) = dir_details_response.pop() {

            let path = self.path.join(&content.name);
            let name  = content.name;

            if content.r#type == "file" {   
                let file = File {
                    _name: name,
                    _hash: content.sha,
                    _size: content.size,
                    url: Url::parse(&content.url.as_str()).unwrap(),
                    // url: Url::parse(content.download_url.as_str()).unwrap(),
                    path,
                };
                self.files.push(file);
            } else if content.r#type == "dir" {
                let mut inner_dir = Directory::new(
                    name, 
                    path, 
                    Url::parse(content.url.as_str()).unwrap()
                );

                inner_dir.update_from_github_api().await;

                self.dirs.push(inner_dir)
            } else {
                panic!("Got unrecognized file type {} from github api", content.r#type)
            }
        }
        
    }

    #[async_recursion]
    pub async fn download_from_github(&self) {
        fs::create_dir_all(&self.path).unwrap();
        
        let files_iter = self.files.iter();
        let inner_dir_iter = self.dirs.iter();

        println!("Downloading files");
        let client = reqwest::Client::new();
        for file in files_iter {
            println!("Downloading file from: {:?}", file.url.as_str());
            match client.get(file.url.as_str())
                    .header(ACCEPT, "application/vnd.github.v3.raw")
                    .send()
                    .await{
                Ok(response) => {
                    let mut file = fs::File::create(&file.path).unwrap();
                    let mut content =  io::Cursor::new(response.bytes().await.unwrap());
                    io::copy(&mut content, &mut file).unwrap();
                },
                Err(e) => {
                    println!("Couldn't download because of {:?}", e);
                }
            }
        };

        println!("Files downloaded");
        println!("Downloading directories");
        for dir in inner_dir_iter {
            dir.download_from_github().await;
        };
        println!("Directories downloaded");

    }

    pub async fn create_zip(&self, final_path: &PathBuf) {
        let tar_gz = fs::File::create(final_path).unwrap();
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        tar.append_dir_all(&self.name, &self.path).unwrap();
    }
}