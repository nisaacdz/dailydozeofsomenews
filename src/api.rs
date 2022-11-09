use std::fmt::Display;

use serde::Deserialize;
use url::Url;
use ureq;

#[allow(dead_code)]
pub struct NewsApi {
    api_key: String,
    address: String,
    pub request: ApiRequest,
    response: Option<ApiResponse>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct News {
    pub title: String,
    pub description: String,
    pub url: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Fils{
    HEADLINES,EVERTHING,
}

#[allow(dead_code)]
pub enum Locs{
    US, JAPAN,CANADA,
}

impl Display for Fils{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Fils::HEADLINES => write!(f, "top-headlines"),
            Fils::EVERTHING => write!(f, "everything"),
        }
    }
}

impl Display for Locs{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Locs::US => write!(f, "us"),
            Locs::JAPAN => write!(f, "japan"),
            Locs::CANADA => write!(f, "canada"),
        }
    }
}

#[allow(dead_code)]
pub struct ApiRequest {
    pub country: Locs,
    pub end_point: Fils,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct ApiResponse {
    status: String,
    articles: Vec<News>,
}

#[allow(dead_code)]
impl NewsApi {
    pub fn new(ad: String, key: String) -> Self {
        Self {
            api_key: key,
            address: ad,
            request: ApiRequest {
                country: Locs::US,
                end_point: Fils::HEADLINES,
            },
            response: None,
        }
    }

    pub fn fetch(& mut self) -> &Vec<News> {
        if let None = self.response{
            self.response = self.querry_api();
        }

        &self.response.as_ref().unwrap().articles
    }

    pub fn fetch_with(&mut self, loc: Locs, point: Fils) -> &Vec<News> {
        self.request.country = loc;
        self.request.end_point = point;

        self.response = None;

        self.fetch()
    }

    fn querry_api(&self) -> Option<ApiResponse> {
        let mut url = Url::parse(self.address.as_str()).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push(&self.request.end_point.to_string());
        url.set_query(Some(&format!("country={}", self.request.country)));

        let url: String = url.to_string();

        let req = ureq::get(&url).set("Authorization", &self.api_key);

        let res: Option<ApiResponse> = req.call().unwrap().into_json().unwrap();

        res
    }

}
