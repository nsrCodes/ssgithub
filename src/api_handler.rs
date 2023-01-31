

use crate::{
    github_client::GithubData, 
    path_parser::RequestMetaData,
    file::get_file_as_byte_vec,
};
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use warp::{
    reply::Response,
    path::FullPath,
    reject,
};

pub async fn api_handler(p: FullPath) -> Result<impl warp::Reply, warp::Rejection> {
    match RequestMetaData::new(&p) {
        Some(request_data) => {
            let (gh_resp, mut result_dir) = GithubData::new(request_data);
        
            result_dir.update_from_github_api().await;
            result_dir.download_from_github().await;
            result_dir.create_zip(&gh_resp.res_path).await;
        
            let mut res = Response::new(get_file_as_byte_vec(&gh_resp.res_path).into());
            println!("gh: {:#?}", &gh_resp);
            tokio::spawn(gh_resp.clean());
            res.headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("application/tar+gzip"));
            Ok(res)
        },
        None => {
            Err(reject::not_found())
        }
    }
}
