use flate2::{Compression, write::GzEncoder};
use futures::future::join_all;
use reqwest::Url;
use std::{
    fs,
    path::PathBuf,
};
use async_recursion::async_recursion;

use crate::github_client::{get_from_github_api, get_file_from_github};

#[derive(Debug)]
pub struct File {
    _name: String,
    pub path: PathBuf,
    pub url: Url,
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
}
impl Directory {
    pub fn new(name: String, path: PathBuf, url: Url) -> Self {
        Directory { 
            name, 
            path, 
            url,
            files: vec![], 
            dirs: vec![], 
        }
    }
    #[async_recursion]
    pub async fn update_from_github_api(&mut self) {
        let dir_details_response = get_from_github_api(&self.url).await;

        for content in dir_details_response.into_iter() {
            let path = self.path.join(&content.name);
            let name  = content.name;

            if content.r#type == "file" {   
                let file = File {
                    _name: name,
                    _hash: content.sha,
                    _size: content.size,
                    url: Url::parse(&content.url.as_str()).unwrap(),
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

        let file_futures = self.files.iter()
            .map(get_file_from_github)
            .collect::<std::vec::Vec<_>>();

        println!("Files downloaded");
        println!("Downloading directories");

        let dir_futures = self.dirs.iter()
            .map(|dir| async {
                dir.download_from_github().await;
            })
            .collect::<std::vec::Vec<_>>();
        
        tokio::join!(
            join_all(file_futures),
            join_all(dir_futures),
        );
        println!("Directories downloaded");
    }

    pub async fn create_zip(&self, final_path: &PathBuf) {
        let tar_gz = fs::File::create(final_path).unwrap();
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        tar.append_dir_all(&self.name, &self.path).unwrap();
    }
}