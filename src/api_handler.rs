// use std::path::{Path, self};
use std::{fs, path::PathBuf};
use std::io::Read;

use crate::{github_client::GithubData, path_parser::RequestMetaData};
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use warp::path::FullPath;
use warp::reply::Response;

fn get_file_as_byte_vec(path: &PathBuf) -> Vec<u8> {
    let mut f = fs::File::open(&path).expect("no file found");
    let metadata = fs::metadata(&path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

pub async fn api_handler(p: FullPath) -> Result<impl warp::Reply, warp::Rejection> {
    let request_data = RequestMetaData::new(&p);
    let (gh_resp, mut result_dir) = GithubData::new(request_data);
    // gh_resp.download_and_zip().await;

    result_dir.update_from_github_api().await;
    result_dir.download_from_github().await;
    result_dir.create_zip().await;

    let mut res = Response::new(get_file_as_byte_vec(&gh_resp.res_path).into());
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/gzip"));
    return Ok(res);
}

#[cfg(test)]
mod api_handler_tests {
    #[test]
    fn zip_and_serve_random_dir_tar_stream() {
        // todo
        // in creater of test create a dir with a random hirearchy of contents
        // validate that the crated zipped tar stream has the same hierarchy of contents
    }
}
