use rnd;
use std::fmt::{Debug, Display};

use serde::Deserialize;
use ureq;
use url::Url;

#[allow(dead_code)]
pub struct NewsApi {
    api_key: String,
    address: String,
    pub request: ApiRequest,
    response: Option<ApiResponse>,
    failure: Vec<News>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct News {
    title: String,
    source: Source,
    author: String,
    description: String,
    url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Source {
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
        &self.description
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    pub fn get_source(&self) -> &Source {
        &self.source
    }

    pub fn mock() -> Self {
        Self {
            title: rnd::write_sentence(30),
            description: rnd::write_sentence(60),
            url: rnd::write_sentence(8),
            source: Source {
                name: rnd::write_word(),
            },
            author: format!("{} {}", rnd::write_word(), rnd::write_word()),
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
    pub code: Option<String>,
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
            failure: vec![News {
                title: "None Available".to_owned(),
                description: "None available".to_owned(),
                url: String::new(),
                source: Source {
                    name: String::new(),
                },
                author: String::new(),
            }],
        }
    }

    pub fn fake_fetch(&mut self) -> &Vec<News> {
        self.mock().fetch().collect()
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
                name: String::new(),
            },
            author: String::new(),
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

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Could not fetch articles")]
    FetchFailed(#[from] ureq::Error),
    #[error("Could not convert articles to string")]
    ParsingFailed(#[from] std::io::Error),
    #[error("Could not parse articles to Articles struct")]
    JsonError(#[from] serde_json::Error),
    #[error("Could not parse given string into url")]
    UrlError(#[from] url::ParseError),
    #[error("Request failed {0}")]
    BadRequest(&'static str),
    #[error("Async fetching failed")]
    AsyncError(#[from] reqwest::Error),
}
