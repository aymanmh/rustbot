use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "res")]
    pub res: String,

    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "size")]
    pub size: String,

    #[serde(rename = "quota_daily_used")]
    pub quota_daily_used: String,

    #[serde(rename = "quota_daily")]
    pub quota_daily: String,

    #[serde(rename = "lang")]
    pub lang: String,

    #[serde(rename = "time")]
    pub time: String,

    #[serde(rename = "query")]
    pub query: String,

    #[serde(rename = "collection")]
    pub collection: String,

    #[serde(rename = "max_score")]
    pub max_score: String,

    #[serde(rename = "hits_total")]
    pub hits_total: String,

    #[serde(rename = "scope")]
    pub scope: String,

    #[serde(rename = "hits")]
    pub hits: Vec<Hit>,
}

#[derive(Serialize, Deserialize)]
pub struct Hit {
    #[serde(rename = "country")]
    pub country: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "pub_day")]
    pub pub_day: String,

    #[serde(rename = "score")]
    pub score: String,

    #[serde(rename = "body")]
    pub body: String,

    #[serde(rename = "pub_year")]
    pub pub_year: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "article_id")]
    pub article_id: String,

    #[serde(rename = "len")]
    pub len: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "pub_month")]
    pub pub_month: String,

    #[serde(rename = "title")]
    pub title: String,
}
