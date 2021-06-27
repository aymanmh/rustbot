use exitfailure::ExitFailure;
use std::env;
use configparser::ini::Ini;
mod google_translate;
mod word_api;
mod word;
mod pressmon_api;
mod pressmon_result;

#[tokio::main]
async fn main() -> Result<(), ExitFailure>
{
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    let mut config = Ini::new();
    config.load("config/config.ini").expect("Error reading config file");

    let google_translate_url = config.get("DEFAULT", "google_translate_url").unwrap();
    let wordapi_url = config.get("DEFAULT", "wordapi_url").unwrap();
    let rapid_api_key = config.get("DEFAULT", "rapid_api_key").unwrap();
    let pressmon_api_key = config.get("DEFAULT", "pressmon_api_key").unwrap();
    assert_ne!(rapid_api_key, "");
    assert_ne!(pressmon_api_key, "");

    let word: String = "Hello world!".to_string();
    //let translator = google_translate::TranslationAPI::new(google_translate_url,rapid_api_key);
    
    //let wordapi = word_api::WordAPI::new(wordapi_url,rapid_api_key);
    let pressmonapi = pressmon_api::PressmonAPI::new(pressmon_api_key);
    
    //let res = google_translate::Translation::post(&word).await?;
    //let res = translator.post(&word).await?;
    let theword: String = "phlegmatic".to_string();
    let res3 = pressmonapi.get(&theword).await?;
    //println!("{:?}",res2.pronunciation.all);
    println!("{:?}",res3.max_score);
    
    Ok(())    
}
