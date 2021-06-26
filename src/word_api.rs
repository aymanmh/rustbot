
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use exitfailure::ExitFailure;
//mod word_structure;

use super::word;

pub struct WordAPI 
{
    api_url:String,
    api_key:String
}

impl WordAPI
{
    pub fn new(url:String, key:String) -> Self
    {
        WordAPI
        {
            api_url:url,
            api_key:key
        }
    }
    pub async fn get(&self, word: &String) -> Result<word::WORDRESULT, ExitFailure>
    {
        let mut headers = HeaderMap::new();
        println!("inside function");
        //headers.insert("content-type","application/x-www-form-urlencoded".parse().unwrap());
        //headers.insert("accept-encoding","application/gzip".parse().unwrap());
        headers.insert("x-rapidapi-key",self.api_key.parse().unwrap());
        headers.insert("x-rapidapi-host","wordsapiv1.p.rapidapi.com".parse().unwrap());
        headers.insert("useQueryString","true".parse().unwrap());

        let mut getURL = self.api_url.clone();
        getURL.push_str(word);

        let result = reqwest::Client::new()
        .get(getURL)
        .headers(headers)
        //.form(&map)
        //.send().await?.json::<serde_json::Value>().await?;
        .send().await?.json::<word::WORDRESULT>().await?;

        //println!("{:#?}", result);

        Ok(result)
    }
}