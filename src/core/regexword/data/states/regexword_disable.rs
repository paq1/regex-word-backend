use crate::core::regexword::data::regexword_data::RegexWordData;
use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::views::{RegexWordDataView, RegexWordDisableView};
use framework_cqrs_lib::cqrs::models::jsonapi::CanGetTypee;
use crate::core::regexword::data::states::regexword_activate::RegexWordActivate;

#[derive(Clone, Debug)]
pub struct RegexWordDisable {
    pub kind: String, // TODO : a challenger l'utilitée
    pub data: RegexWordData,
}

impl RegexWordDisable {
    pub fn reduce_state(&self, event: RegexWordEvents) -> Option<RegexWordStates> {
        match event {
            RegexWordEvents::Activated(event) => Some(
                RegexWordStates::RegexWordActivate(
                    RegexWordActivate {
                        kind: self.kind.clone(),
                        data: self.data.clone(),
                        date_activate: event.date_activate
                    }
                )
            ),
            _ => None // illegal transition
        }
    }
}

impl CanGetTypee for RegexWordDisable {
    fn get_type(&self) -> String {
        "urn:api:regexword:regexword".to_string()
    }
}

impl From<RegexWordDisable> for RegexWordDisableView {
    fn from(value: RegexWordDisable) -> Self {
        RegexWordDisableView {
            data: RegexWordDataView {
                regex_parts: value.data.regex_parts,
                word: value.data.word,
                niveau_difficulte: value.data.niveau_difficulte,
            }
        }
    }
}
