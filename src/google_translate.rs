
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use exitfailure::ExitFailure;


#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleTranslate 
{
    data: Translations,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Translations
{
    translations: Vec<Translation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Translation 
{
    translated_text: String,
}

/*
{\"data\":
	{\"translations\":
		[
			{
			\"translatedText\":\"┬íHola Mundo!\"
			}
		]
	}
}
*/

pub struct TranslationAPI 
{
    api_url:String,
    api_key:String
}

impl TranslationAPI
{
    pub fn new(url:String, key:String) -> Self
    {
        TranslationAPI
        {
            api_url:url,
            api_key:key
        }
    }
    pub async fn post(&self, word: &String) -> Result<Vec<String>, ExitFailure>
    {
        let target_lng:String = "es".to_string();
        let source_lang:String = "en".to_string();

        let mut headers = HeaderMap::new();
        
        headers.insert("content-type","application/x-www-form-urlencoded".parse().unwrap());
        headers.insert("accept-encoding","application/gzip".parse().unwrap());
        headers.insert("x-rapidapi-key",self.api_key.parse().unwrap());
        headers.insert("x-rapidapi-host","google-translate1.p.rapidapi.com".parse().unwrap());
        headers.insert("useQueryString","true".parse().unwrap());

        let mut map = HashMap::new();
        map.insert("q",word);
        map.insert("target", &target_lng);
        map.insert("source", &source_lang);
        
        let result = reqwest::Client::new()
        .post(self.api_url.clone())
        .headers(headers)
        .form(&map)
        .send().await?.json::<GoogleTranslate>().await?;

        let mut translation_results: Vec<String> = Vec::new();
        for entry in result.data.translations.iter()
        {
            translation_results.push(entry.translated_text.clone());
        }

        Ok(translation_results)
    }
}