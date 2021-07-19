use exitfailure::ExitFailure;
use reqwest::header::HeaderMap;
use std::env;

use super::word_result;

pub async fn get(word: &String) -> Result<word_result::WORDRESULT, ExitFailure> {
//pub async fn get(word: &String) -> Result<(), ExitFailure> {
    let api_key = env::var("rapid_api_key").unwrap();
    let mut api_url = String::from("https://wordsapiv1.p.rapidapi.com/words/");
    assert_ne!(api_key, "");

    let mut headers = HeaderMap::new();

    headers.insert("x-rapidapi-key", api_key.parse().unwrap());
    headers.insert("x-rapidapi-host","wordsapiv1.p.rapidapi.com".parse().unwrap());
    headers.insert("useQueryString", "true".parse().unwrap());

    if !word.is_empty()
    {
        api_url.push_str(word);
    }
    else
    {
        api_url.push_str("?random=true");
    }
    println!("{}",api_url);

    let result = reqwest::Client::new()
        .get(api_url)
        .headers(headers)
        .send()
        .await?.json::<word_result::WORDRESULT>().await?;
    //println!("{}",result.status());

    //let serade_res = result.json::<word_result::WORDRESULT>()
    //    .await;
    //let serade_res = result.json::<serde_json::Value>()
    //    .await?;
    //println!("{:?}",serade_res);

    //println!("{}",serade_res.pronunciation.all);
    //let result3 = result.json::<serde_json::Value>()
     //   .await?;
    //println!("{:?}",serade_res);
    //let serade_res:word_result::WORDRESULT = serde_json::from_value(result3);
    Ok(result)
    //Ok(())
}
