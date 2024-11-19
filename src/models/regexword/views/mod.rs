use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RegexWordDataView {
    pub regex_parts: Vec<String>,
    pub word: String,
    pub niveau_difficulte: String,
    pub nb_selected: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum RegexWordViewState {
    #[serde(rename = "regexword")]
    Activate(RegexWordView),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegexWordView {
    #[serde(flatten)]
    pub data: RegexWordDataView,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum RegexWordViewEvent {
    #[serde(rename = "created")]
    Created(RegexWordCreatedView),
    #[serde(rename = "activate")]
    Activated(RegexWordIncrementedView),
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RegexWordCreatedView {
    #[serde(flatten)]
    pub data: RegexWordDataView,
    pub by: String,
    pub at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RegexWordIncrementedView {
    pub by: String,
    pub at: DateTime<Utc>,
    pub nb_selected: u32,
}

