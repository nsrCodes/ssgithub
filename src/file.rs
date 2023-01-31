use std::{fs, path::PathBuf};
use std::io::{Read, self};
use reqwest::Url;
use reqwest::header::ACCEPT;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub url: Url,
    pub hash: String,
    pub size: u32,
}

pub fn get_file_as_byte_vec(path: &PathBuf) -> Vec<u8> {
    let mut f = fs::File::open(&path).expect("no file found");
    let metadata = fs::metadata(&path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
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