use crate::api::regexword::regexword_dbo::RegexWordDataDbo;
use crate::core::regexword::data::regexword_data::RegexWordData;

pub mod states;
pub mod events;

impl From<RegexWordDataDbo> for RegexWordData {
    fn from(value: RegexWordDataDbo) -> Self {
        Self {
            word: value.word,
            regex_parts: value.regex_parts,
            niveau_difficulte: value.niveau_difficulte,
        }
    }
}

impl From<RegexWordData> for RegexWordDataDbo {
    fn from(value: RegexWordData) -> Self {
        Self {
            word: value.word,
            regex_parts: value.regex_parts,
            niveau_difficulte: value.niveau_difficulte,
        }
    }
}

