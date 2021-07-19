use serde::{Deserialize, Serialize};
use serde_json::Number;

    #[derive(Debug, Deserialize)]
    pub struct WORDRESULT {
        #[serde(rename = "pronunciation")]
        pub pronunciation: Option<Pronunciation>,

        #[serde(rename = "results")]
        pub results: Option<Vec<Result>>,

        #[serde(rename = "syllables")]
        pub syllables: Option<Syllables>,

        #[serde(rename = "word")]
        pub word: String,

        #[serde(rename = "frequency")]
        pub frequency: Option<f64>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Pronunciation {
        #[serde(rename = "all")]
        pub all: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Result {
        #[serde(rename = "definition")]
        pub definition: Option<String>,

        #[serde(rename = "derivation")]
        pub derivation: Option<Vec<String>>,

        #[serde(rename = "examples")]
        pub examples: Option<Vec<String>>,

        #[serde(rename = "partOfSpeech")]
        pub part_of_speech: Option<String>,

        #[serde(rename = "similarTo")]
        pub similar_to: Option<Vec<String>>,

        #[serde(rename = "synonyms")]
        pub synonyms: Option<Vec<String>>,

        #[serde(rename = "cause")]
        pub cause: Option<Vec<String>>,

        #[serde(rename = "antonyms")]
        pub antonyms: Option<Vec<String>>,


        #[serde(rename = "typesOf")]
        pub types_of: Option<Vec<String>>,

        #[serde(rename = "hasTypes")]
        pub has_types: Option<Vec<String>>,

    }

    #[derive(Debug, Deserialize)]
    pub struct Syllables {
        #[serde(rename = "count")]
        pub count: i64,

        #[serde(rename = "list")]
        pub list: Vec<String>,
    }
