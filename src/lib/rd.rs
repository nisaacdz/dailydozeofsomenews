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

pub async fn read_req(api: &mut Api) -> Result<&mut Api, MyErrs> {
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

pub fn read(api: &mut Api) -> Result<&mut Api, MyErrs> {
    let cn = api.request.country;
    let ep = api.request.end_point;

    match api.get_response() {
        Some(_) => {
            let query = api.get_query();
            if query.1 != cn || query.0 != ep {
                query.1 = cn;
                query.0 = ep;
                return read_ureq(api);
            } else {
                return Ok(api);
            }
        }
        None => return read_ureq(api),
    }
}

pub fn read_ureq(api: &mut Api) -> Result<&mut Api, MyErrs> {
    let url = prep_url(api)?;

    let res = ureq::get(&url).set("Authorization", &get_key());

    let res = res.call().map_err(|_| MyErrs::UreqError)?;

    let res: ApiResponse = res.into_json().map_err(|_| MyErrs::JsonParseErr)?;

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
    String::from("ce262f1d2c1a4288a8960760763fc0b1")
}
