use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BandeauDataDbo {
    pub word: String,
    pub regex_parts: Vec<String>,
    pub niveau_difficulte: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RegexWordDboState {
    BandeauCreatedDbo {
        #[serde(rename = "_kind")]
        kind: String,
        #[serde(flatten)]
        data: BandeauDataDbo,
    },
    BandeauProdDbo {
        #[serde(rename = "_kind")]
        kind: String,
        #[serde(flatten)]
        data: BandeauDataDbo,
    },
    BandeauDisableDbo {
        #[serde(rename = "_kind")]
        kind: String,
        #[serde(flatten)]
        data: BandeauDataDbo,
        reason: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RegexWordDboEvent {
    Created(BandeauCreatedDbo),
    Updated(BandeauUpdatedDbo),
    Disable(BandeauDisabledDbo),
    Prod(BandeauProdUpDbo),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BandeauCreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: BandeauDataDbo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BandeauProdUpDbo {
    pub by: String,
    pub at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BandeauUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: BandeauDataDbo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BandeauDisabledDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub reason: String,
}

