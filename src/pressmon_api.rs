use exitfailure::ExitFailure;
use reqwest::header::HeaderMap;
use std::env;

use super::pressmon_result;

pub async fn get(word: &String) -> Result<pressmon_result::Result, ExitFailure> {
    let mut headers = HeaderMap::new();

    let api_key = env::var("pressmon_api_key").unwrap();
    let mut api_url = String::from("https://pressmon.com/api?q=");
    assert_ne!(api_key, "");

    headers.insert("useQueryString", "true".parse().unwrap());
    //https://pressmon.com/api?q=word-to-search&key=YOUR_API_KEY

    api_url.push_str(word);
    api_url.push_str("&size=3");
    api_url.push_str("&key=");
    api_url.push_str(&api_key);
    //println!("{}", api_url);

    let result = reqwest::Client::new()
        .get(api_url)
        .headers(headers)
        .send()
        .await?
        .json::<pressmon_result::Result>()
        .await?;

    Ok(result)
}
