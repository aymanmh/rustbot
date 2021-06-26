use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct WORDRESULT {
        #[serde(rename = "pronunciation")]
        pub pronunciation: Pronunciation,

        #[serde(rename = "results")]
        pub results: Vec<Result>,

        #[serde(rename = "syllables")]
        pub syllables: Syllables,

        #[serde(rename = "word")]
        pub word: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Pronunciation {
        #[serde(rename = "all")]
        pub all: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Result {
        #[serde(rename = "definition")]
        pub definition: String,

        #[serde(rename = "derivation")]
        pub derivation: Vec<String>,

        #[serde(rename = "examples")]
        pub examples: Vec<String>,

        #[serde(rename = "partOfSpeech")]
        pub part_of_speech: String,

        #[serde(rename = "similarTo")]
        pub similar_to: Vec<String>,

        #[serde(rename = "synonyms")]
        pub synonyms: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Syllables {
        #[serde(rename = "count")]
        pub count: i64,

        #[serde(rename = "list")]
        pub list: Vec<String>,
    }
