

use crate::file::get_file_as_byte_vec;
use crate::{github_client::GithubData, path_parser::RequestMetaData};
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use warp::path::FullPath;
use warp::reply::Response;



pub async fn api_handler(p: FullPath) -> Result<impl warp::Reply, warp::Rejection> {
    let request_data = RequestMetaData::new(&p);
    let (gh_resp, mut result_dir) = GithubData::new(request_data);

    result_dir.update_from_github_api().await;
    result_dir.download_from_github().await;
    result_dir.create_zip(&gh_resp.res_path).await;

    let mut res = Response::new(get_file_as_byte_vec(&gh_resp.res_path).into());
    println!("gh: {:#?}", &gh_resp);
    tokio::spawn(gh_resp.clean());
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/tar+gzip"));
    return Ok(res);
}
