use std::fmt::{Display, Debug};

use serde::Deserialize;
use ureq;
use url::Url;

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
    title: String,
    description: String,
    url: String,
}

impl News {
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_desc(&self) -> &str {
        &self.description
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn mock() -> Self {
        Self {
            title: "A bunch of unknown projects are being undertaken by teh Metropolitan institute A bunch of unknown projects are being undertaken by teh Metropolitan institute".to_owned(),
            description: "Sorts a Vector of Person structs with properties name and age by its natural order (By name and age). In order to make Person sortable you need four traits Eq, PartialEq, Ord and PartialOrd. These traits can be simply derived. You can also provide a custom comparator function using a vec:sort_by method and sort only by age.".to_owned(),
            url: "helloworldatcnbc@gmail.com/dailynews/api/random/everuthing/institute".to_owned(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Fils {
    HEADLINES,
    EVERTHING,
}

#[allow(dead_code)]
pub enum Locs {
    US,
    JAPAN,
    CANADA,
}

impl Display for Fils {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fils::HEADLINES => write!(f, "top-headlines"),
            Fils::EVERTHING => write!(f, "everything"),
        }
    }
}

impl Debug for Fils {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HEADLINES => write!(f, "HEADLINES"),
            Self::EVERTHING => write!(f, "ALL"),
        }
    }
}

impl Display for Locs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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

    pub fn mock(&mut self) -> &Vec<News> {
        let mut vec: Vec<News> = Vec::new();

        for _ in 0..100 {
            vec.push(News::mock());
        }

        self.response = Some(ApiResponse{
            status: "ok".to_owned(),
            articles: vec,
        });

        &self.response.as_ref().unwrap().articles
    }

    pub fn fetch(&mut self) -> &Vec<News> {
        if let None = self.response {
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
