
use reqwest::header::HeaderMap;
use exitfailure::ExitFailure;

use super::pressmon_result;

pub struct PressmonAPI 
{
    api_url:String,
    api_key:String
}

impl PressmonAPI
{
    pub fn new(key:String) -> Self
    {
        PressmonAPI
        {
            api_url:"https://pressmon.com/api?q=".to_string(),
            api_key:key
        }
    }
    
    pub async fn get(&self, word: &String) -> Result<pressmon_result::Result, ExitFailure>
    {
        let mut headers = HeaderMap::new();
        
        headers.insert("useQueryString","true".parse().unwrap());
        //https://pressmon.com/api?q=word-to-search&key=YOUR_API_KEY
        let mut getURL = self.api_url.clone();
        getURL.push_str(word);
        getURL.push_str("&size=3");
        getURL.push_str("&key=");
        getURL.push_str(&self.api_key);
        println!("{}", getURL);

        let result = reqwest::Client::new()
        .get(getURL)
        .headers(headers)
        .send().await?.json::<pressmon_result::Result>().await?;

                 
        Ok(result)
    }
}