use reqwest::Url;
use warp::path::FullPath;

#[derive(Debug, PartialEq)]
pub struct RequestMetaData {
    pub username: String,
    pub repo: String,
    pub path: String,
    pub branch: String,
}
impl RequestMetaData {
    fn validate_path(raw_path: &FullPath) {
        // expects "/{username}/{repo}/tree/{branch}/{path....}"
        
        let raw_path_vec: Vec<&str> = raw_path.as_str().split("/").collect();
        if raw_path_vec.is_empty() {
            panic!("Empty path recieved")
        }
        let mut path_vec: Vec<&str> = raw_path.as_str().split("/").collect();

        if path_vec[0].is_empty() {
            path_vec.remove(0);
        }

        if path_vec.len() < 4 {
            panic!("Path length too small")
        }

        if !path_vec.contains(&"tree") {
            panic!("Path seems invalid, does not contains `tree` as subpath")
        }
    }

    pub fn new(raw_path: &FullPath) -> Self {
        Self::validate_path(raw_path); // will panic if invalid path
        let mut path_vec: Vec<&str> = raw_path.as_str().split("/").collect();

        if path_vec[0].is_empty() {
            path_vec.remove(0);
        }

        RequestMetaData {
            username: String::from(path_vec[0]),
            repo: String::from(path_vec[1]),
            branch: String::from(path_vec[3]),
            path: path_vec[4..].to_vec().join("/"),
        }
    }
    pub fn api_target(&self) -> Url {
        let mut url_str = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.username, self.repo, self.path
        );
        if self.branch != "master" && self.branch != "main" && !self.branch.is_empty() {
            url_str = format!("{url_str}?ref={}", self.branch);
        };
        
        let res = Url::parse(url_str.as_str()).unwrap();
        res
    }
}

#[cfg(test)]
mod path_parser_tests {
    use crate::path_parser::RequestMetaData;
    use reqwest::Url;
    use warp::{path::FullPath, Filter};

    #[tokio::test]
    async fn pathmetadata_new_constructor() {
        let full_path_filter = warp::any().and(warp::path::full()).map(|p: FullPath| {
            print!("testing with path {:?}", p);
            RequestMetaData::new(&p)
        });

        // test 1
        let value_1 = warp::test::request()
            .path("/spur-dev/spur/tree/master/src")
            .filter(&full_path_filter)
            .await
            .unwrap();

        let expected_res_1 = RequestMetaData {
            username: String::from("spur-dev"),
            repo: String::from("spur"),
            path: String::from("src"),
            branch: String::from("master"),
        };

        assert_eq!(value_1, expected_res_1);
    }

    #[test]
    fn api_target_non_main_branch_gives_correct_url() {
        let dummy_meta_data = RequestMetaData {
            username: String::from("joey"),
            repo: String::from("pizza"),
            path: String::from("src/monica/fridge.json"),
            branch: String::from("not-main"),
        };

        let url = Url::parse(
            "https://api.github.com/repos/joey/pizza/contents/src/monica/fridge.json?ref=not-main",
        )
        .unwrap();
        assert_eq!(dummy_meta_data.api_target(), url);
    }
    #[test]
    fn api_target_main_branch_gives_correct_url() {
        let dummy_meta_data = RequestMetaData {
            username: String::from("joey"),
            repo: String::from("pizza"),
            path: String::from("src/monica/fridge.json"),
            branch: String::from("main"),
        };

        let url =
            Url::parse("https://api.github.com/repos/joey/pizza/contents/src/monica/fridge.json")
                .unwrap();
        assert_eq!(dummy_meta_data.api_target(), url);
    }
}
