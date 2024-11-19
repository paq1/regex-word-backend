use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordDataDbo {
    pub word: String,
    pub regex_parts: Vec<String>,
    pub niveau_difficulte: String,
    pub nb_selected: u32,
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
    Incremented(RegexWordIncrementedDbo),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordCreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: RegexWordDataDbo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegexWordIncrementedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub nb_selected: u32,
}
