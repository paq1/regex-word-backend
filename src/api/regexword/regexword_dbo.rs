use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordDataDbo {
    pub word: String,
    pub regex_parts: Vec<String>,
    pub niveau_difficulte: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RegexWordDboState {
    RegexWordActivateDbo {
        #[serde(rename = "_kind")]
        kind: String,
        #[serde(flatten)]
        data: RegexWordDataDbo,
        date_activate: NaiveDate,
    },
    RegexWordDisableDbo {
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
    Activate(RegexWordActivateDbo),
    Disable(RegexWordDisabledDbo),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordCreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: RegexWordDataDbo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordActivateDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub date_activate: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordDisabledDbo {
    pub by: String,
    pub at: DateTime<Utc>,
}

