use std::fmt::Debug;
use std::fmt::Display;

use crate::rnd;
use serde::Deserialize;

pub struct Api {
    pub request: ApiRequest,
    pub response: Option<ApiResponse>,
    query: (Fils, Locs),
}

impl Api {
    pub fn new() -> Self {
        Self {
            request: ApiRequest {
                country: Locs::US,
                end_point: Fils::HEADLINES,
            },
            response: None,
            query: (Fils::HEADLINES, Locs::US),
        }
    }

    pub fn get_request(&self) -> &ApiRequest {
        &self.request
    }

    pub fn get_request_mut(&mut self) -> &mut ApiRequest {
        &mut self.request
    }

    pub fn get_response(&self) -> Option<&ApiResponse> {
        if let None = self.response {
            return None;
        }

        Some(self.response.as_ref().unwrap())
    }

    pub fn get_query(&mut self) -> &mut (Fils, Locs) {
        &mut self.query
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum Fils {
    HEADLINES,
    EVERTHING,
}

impl Fils {
    const ALL: [Fils; 2] = [Fils::HEADLINES, Fils::EVERTHING];

    pub fn get_endpoints() -> &'static [Fils; 2] {
        &Fils::ALL
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Locs {
    US,
    JAPAN,
    CANADA,
    GREATBRITAIN,
    CHINA,
    RUSSIA,
}

#[allow(dead_code)]
impl Locs {
    const ALL: [Locs; 6] = [
        Locs::US,
        Locs::JAPAN,
        Locs::CANADA,
        Locs::CHINA,
        Locs::GREATBRITAIN,
        Locs::RUSSIA,
    ];

    pub fn get_countries() -> &'static [Locs; 6] {
        &Locs::ALL
    }
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
            Locs::JAPAN => write!(f, "jp"),
            Locs::CANADA => write!(f, "ca"),
            Locs::GREATBRITAIN => write!(f, "gb"),
            Locs::RUSSIA => write!(f, "rs"),
            Locs::CHINA => write!(f, "cn"),
        }
    }
}

#[allow(dead_code)]
pub struct ApiRequest {
    pub country: Locs,
    pub end_point: Fils,
}

impl ApiRequest {
    pub fn ep(&mut self) -> &mut Fils {
        &mut self.end_point
    }

    pub fn cn(&mut self) -> &mut Locs {
        &mut self.country
    }
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct News {
    title: String,
    source: Source,
    author: Option<String>,
    description: Option<String>,
    url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Source {
    id: Option<String>,
    name: String,
}

#[allow(dead_code)]
impl Source {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl News {
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_desc(&self) -> &str {
        if let Some(v) = &self.description {
            return v;
        }

        "None Available"
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_author(&self) -> &str {
        if let None = self.author {
            return "";
        }

        &self.author.as_ref().unwrap()
    }

    pub fn get_source(&self) -> &Source {
        &self.source
    }

    pub fn mock() -> Self {
        Self {
            title: rnd::write_sentence(30),
            description: Some(rnd::write_sentence(60)),
            url: rnd::write_sentence(8),
            source: Source {
                id: None,
                name: rnd::write_word(),
            },
            author: Some(format!("{} {}", rnd::write_word(), rnd::write_word())),
        }
    }
}

/*


*/

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct ApiResponse {
    status: String,
    articles: Vec<News>,
}

impl ApiResponse {
    pub fn new(s: String, vec: Vec<News>) -> Self {
        Self {
            status: s,
            articles: vec,
        }
    }

    pub fn get_articles(&self) -> &Vec<News> {
        &self.articles
    }
}

/*
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
            failure: vec![News {
                title: "None Available".to_owned(),
                description: "None available".to_owned(),
                url: String::new(),
                source: Source {
                    id: None,
                    name: String::new(),
                },
                author: None,
            }],
        }
    }

    pub fn fake_fetch(&mut self) -> &Vec<News> {
        self.mock().fetch().collect()
    }

    pub fn real_fetch(&mut self) -> &Vec<News> {
        self.fetch().collect()
    }

    pub fn mock(&mut self) -> &mut Self {
        if let None = self.response {
            let mut vec: Vec<News> = Vec::new();

            for _ in 0..200 {
                vec.push(News::mock());
            }

            self.response = Some(ApiResponse {
                status: "ok".to_owned(),
                code: Some("apiKeyDisabled".to_owned()),
                articles: vec,
            });
        }

        self
    }

    pub fn collect(&self) -> &Vec<News> {
        let res = self.response.as_ref();

        if let None = res {
            return &self.failure;
        }

        let res = res.unwrap();

        match res.status.as_str() {
            "ok" => return &self.response.as_ref().unwrap().articles,
            _ => return &self.failure,
        };
    }

    pub fn fetch(&mut self) -> &Self {
        if let None = self.response {
            let res = self.querry_api();

            match res {
                Ok(val) => match val.status.as_str() {
                    "ok" => self.response = Some(val),
                    _ => self.map_request_error(&val.code),
                },
                Err(e) => self.write_error(e),
            }
        }
        self
    }

    fn write_error(&mut self, err: ApiError) {
        self.failure = vec![News {
            title: format!("{:?}", err),
            description: "Nothing available".to_owned(),
            url: "".to_owned(),
            source: Source {
                id: None,
                name: String::new(),
            },
            author: None,
        }];
    }

    pub fn fetch_with(&mut self, loc: Locs, point: Fils) -> &Vec<News> {
        self.request.country = loc;
        self.request.end_point = point;

        self.response = None;

        self.fetch().collect()
    }

    fn querry_api(&self) -> Result<ApiResponse, ApiError> {
        let mut url = Url::parse(self.address.as_str()).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push(&self.request.end_point.to_string());
        url.set_query(Some(&format!("country={}", self.request.country)));

        let url: String = url.to_string();

        let req = ureq::get(&url).set("Authorization", &self.api_key);

        let res: ApiResponse = req.call()?.into_json()?;

        Ok(res)
    }

    */

/*

    pub fn map_request_error(&mut self, code: &Option<String>) {
        if let Some(code) = code {
            match code.as_str() {
                "apiKeyDisabled" => {
                    self.write_error(ApiError::BadRequest("Api key disabled"));
                }
                _ => {
                    self.write_error(ApiError::BadRequest("Unknown Error"));
                }
            }
        } else {
            self.write_error(ApiError::BadRequest("Unknown Error"));
        }
    }
}

*/
