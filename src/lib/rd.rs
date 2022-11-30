use dotenv::dotenv;
use reqwest::{Response as ReqResponse, Url};

use crate::{
    api::{Api, ApiResponse, News},
    ers::MyErrs,
};
const BASE: &str = "https://newsapi.org/v2";

pub fn read_f(api: &mut Api) -> Result<&mut Api, MyErrs> {
    if let None = api.response {
        let mut vec: Vec<News> = Vec::new();

        for _ in 0..200 {
            vec.push(News::mock());
        }
        api.response = Some(ApiResponse::new("ok".to_owned(), vec));
    }
    Ok(api)
}

pub async fn read(api: &mut Api) -> Result<&mut Api, MyErrs> {
    dotenv().map_err(|_| MyErrs::CannotFindEnv)?;

    let url = prep_url(api)?;

    let client = reqwest::Client::new();

    let res: ReqResponse = client
        .get(url)
        .header("Authorization", get_key())
        .send()
        .await
        .map_err(|_| MyErrs::BadFeedback)?;

    let res: ApiResponse = res.json().await.map_err(|e| MyErrs::JsonParseFailed(e))?;

    api.response = Some(res);

    Ok(api)
}

pub fn read_ureq(api: &mut Api) -> Result<&mut Api, MyErrs> {
    let url =
        "https://newsapi.org/v2/top-headlines?country=us&apiKey=ce262f1d2c1a4288a8960760763fc0b1";
    let res = ureq::get(url).call().map_err(|_| MyErrs::UreqError)?;

    let res = res.into_string().map_err(|_| MyErrs::UreqError)?;

    let res: ApiResponse = serde_json::from_str(&res).map_err(|e| MyErrs::JsonParseErr(e))?;
    api.response = Some(res);

    Ok(api)
}

fn prep_url(api: &Api) -> Result<String, MyErrs> {
    let req = api.get_request();

    let mut url = Url::parse(BASE).map_err(|_| MyErrs::UrlParseFailed)?;
    url.path_segments_mut()
        .unwrap()
        .push(&req.end_point.to_string());
    url.set_query(Some(&format!("country={}", req.country.to_string())));
    Ok(url.to_string())
}

pub fn get_key() -> String {
    std::env::var("API_KEY").unwrap()
}
