use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RegexWordDataView {
    pub regex_parts: Vec<String>,
    pub word: String,
    pub niveau_difficulte: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum RegexWordViewState {
    #[serde(rename = "activate")]
    Activate(RegexWordActivateView),
    #[serde(rename = "disable")]
    Disable(RegexWordDisableView),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegexWordActivateView {
    #[serde(flatten)]
    pub data: RegexWordDataView,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegexWordDisableView {
    #[serde(flatten)]
    pub data: RegexWordDataView,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum RegexWordViewEvent {
    #[serde(rename = "created")]
    Created(RegexWordCreatedView),
    #[serde(rename = "activate")]
    Activated(RegexWordActivatedView),
    #[serde(rename = "disabled")]
    Disabled(RegexWordDisabledView),
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RegexWordCreatedView {
    #[serde(flatten)]
    pub data: RegexWordDataView,
    pub by: String,
    pub at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RegexWordActivatedView {
    pub by: String,
    pub at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RegexWordDisabledView {
    pub by: String,
    pub at: DateTime<Utc>,
}
