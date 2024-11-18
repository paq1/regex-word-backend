use crate::core::regexword::data::regexword_data::RegexWordData;
use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::states::regexword_disable::RegexWordDisable;
use crate::core::regexword::data::states::regexword_prod::BandeauProd;
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::views::{RegexWordDataView, RegexWordActivateView};
use framework_cqrs_lib::cqrs::models::jsonapi::CanGetTypee;

#[derive(Clone, Debug)]
pub struct RegexWordActivate {
    pub kind: String,
    pub data: RegexWordData,
}

impl RegexWordActivate {
    pub fn reduce_state(&self, event: RegexWordEvents) -> Option<RegexWordStates> {
        match event {
            RegexWordEvents::Disabled(disabled) => Some(
                RegexWordStates::RegexWordDisable(
                    RegexWordDisable {
                        kind: self.kind.clone(),
                        data: self.data.clone(),
                    }
                )
            ),
            _ => None // illegal transition
        }
    }
}

impl CanGetTypee for RegexWordActivate {
    fn get_type(&self) -> String {
        "urn:api:regexword:regexword".to_string()
    }
}

impl From<RegexWordActivate> for RegexWordActivateView {
    fn from(value: RegexWordActivate) -> Self {
        RegexWordActivateView {
            data: value.data.into()
        }
    }
}

impl From<RegexWordData> for RegexWordDataView {
    fn from(value: RegexWordData) -> Self {
        Self {
            regex_parts: value.regex_parts,
            word: value.word,
            niveau_difficulte: value.niveau_difficulte,
        }
    }
}

