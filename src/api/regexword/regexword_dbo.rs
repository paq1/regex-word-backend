use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordDataDbo {
    pub word: String,
    pub regex_parts: Vec<String>,
    pub order: Vec<u32>,
    pub nb_selected: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_last_selected: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RegexWordDboState {
    RegexWordDbo {
        #[serde(rename = "_kind")]
        kind: String,
        #[serde(flatten)]
        data: RegexWordDataDbo,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RegexWordDboEvent {
    Created(RegexWordCreatedDbo),
    Selected(RegexWordSelectedDbo),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordCreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: RegexWordDataDbo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordSelectedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub nb_selected: u32,
    pub date_last_selected: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_parts: Option<Vec<String>>,
}
