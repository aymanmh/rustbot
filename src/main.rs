use exitfailure::ExitFailure;
use std::env;
use configparser::ini::Ini;
mod google_translate;
mod word_api;
mod word_result;
mod pressmon_api;
mod pressmon_result;


use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Show examples of the word from press sources")]
    Usage(String),
    #[command(description = "Translate a word to a target language.", parse_with = "split")]
    Translate { word: String, target: String },
    #[command(description = "Get word info.")]
    Word { word: String }
}

async fn getWord(word: &String) -> String 
{
    let mut response = String::from("");
            
    let res = word_api::get(&word).await;

    println!("{}",res.is_err());

    let unwrapped_res:word_result::WORDRESULT;
    if !res.is_err()
    {  
        let unwrapped_res = res.unwrap();
        let mut pronunciation = String::new();
        match unwrapped_res.pronunciation
        {
            Some(value) => pronunciation = String::from(format!("Pronunciation: {}\n", value.all)),
            None        =>  pronunciation = String::from("Pronunciation: N/A\n"),
        }
        response.push_str(&unwrapped_res.word);
        response.push_str("\n");
        response.push_str(&pronunciation);
        let mut freq = String::from("Frequency: ");
        match unwrapped_res.frequency 
        {
            Some(p) => freq.push_str(&p.to_string()),
            None => freq.push_str("N/A"),
        }
        freq.push_str("\n");
        response.push_str(&freq);
        
        match unwrapped_res.results 
        {
            Some(allresults) => //|_|
            {        
                for mut entry in allresults
                {
                    let tempstr = String::from(format!("{} : {}\n", entry.part_of_speech.get_or_insert(String::from("N/A\n")),entry.definition.get_or_insert(String::from("N/A\n"))));
                    response.push_str(&tempstr);
                }
            },
            None => response.push_str("N/A"),
        }
            
        //println!("{}",response);
    }
    else
    {
        response = String::from("Error - Could not get word info :(");
    }

    return response;
}

async fn answer(cx: UpdateWithCx<AutoSend<Bot>, Message>,command: Command,) -> Result<(), Box<dyn Error + Send + Sync>> 
{
    match command 
    {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Usage(word) => 
        {
            let mut response = String::from("");
            
            let res = pressmon_api::get(&word).await;
            let unwrapped_res:pressmon_result::Result;
            if !res.is_err()
            {
                let unwrapped_res = res.unwrap();
                for hit in unwrapped_res.hits
                {
                    let tempstr = String::from(format!("Source: {} - {}\n", hit.source,hit.body));
                    response.push_str(&tempstr);
                }            
                //println!("{}",response);
            }
            else
            {
                response = String::from("Error - Could not get examples :(");
            }

            cx.answer(response).await?
        }
        Command::Translate { word, target } => 
        {
            let mut unwrapped_res:Vec<String> = Vec::new();
            let res = google_translate::post(&word,&target).await;
            if !res.is_err()
            {
                unwrapped_res = res.unwrap();
            }
            else
            {
                unwrapped_res.push("Error - Could not get translations :(".to_string());
            }
            cx.answer(format!("{:?}", unwrapped_res)).await?
        }
        Command::Word { word } => 
        {
            cx.answer(getWord(&word).await).await?            
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() 
{


    let mut config = Ini::new();
    config.load("config/config.ini").expect("Error reading config file");
   
    let rapid_api_key = config.get("DEFAULT", "rapid_api_key").unwrap();
    let pressmon_api_key = config.get("DEFAULT", "pressmon_api_key").unwrap();
    assert_ne!(rapid_api_key, "");
    assert_ne!(pressmon_api_key, "");
    env::set_var("rapid_api_key", rapid_api_key);
    env::set_var("pressmon_api_key", pressmon_api_key);
    
    run().await;
    //test().await;
}

async fn run() 
{
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name = String::from("Rustylang_bot");
    teloxide::commands_repl(bot, bot_name, answer).await;
}

async fn test() 
{
    //let word: String = "phlegmatic".to_string();
    let word: String = String::from("");
    //let word: String = "cool".to_string();
    //let translator = google_translate::TranslationAPI::new(google_translate_url,rapid_api_key);
    
    //let wordapi = word_api::WordAPI::new(wordapi_url,rapid_api_key);
    //let pressmonapi = pressmon_api::PressmonAPI::new(pressmon_api_key);
    //let target_lang = String::from("es");
    //let res = google_translate::post(&word,&target_lang).await;
    //let test = format!("{:?}", res.unwrap());
    //println!("{:?}",res);
    //println!("{:?}",test);

    
    println!("{}",getWord(&word).await);

    //let res2 = word_api::get(&word).await?;
    //println!("{:?}",res2.pronunciation.all);

    //let res3 = pressmon_api::get(&word).await;
    //println!("{:?}",res3.max_score);
}
