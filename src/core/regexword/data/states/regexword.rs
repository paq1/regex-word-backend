use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::regexword_data::RegexWordData;
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::views::{RegexWordDataView, RegexWordView};
use framework_cqrs_lib::cqrs::models::jsonapi::CanGetTypee;

#[derive(Clone, Debug)]
pub struct RegexWord {
    pub kind: String,
    pub data: RegexWordData,
}

impl RegexWord {
    pub fn reduce_state(&self, event: RegexWordEvents) -> Option<RegexWordStates> {
        match event {
            RegexWordEvents::Selected(evt) => Some(
                RegexWordStates::RegexWord(
                    RegexWord {
                        kind: self.kind.clone(),
                        data: RegexWordData {
                            nb_selected: evt.nb_selected,
                            date_last_selected: Some(evt.date_last_selected),
                            regex_parts: evt.regex_parts.unwrap_or(self.data.regex_parts.clone()),
                            ..self.data.clone()
                        },
                    }
                )
            ),
            _ => None // illegal transition
        }
    }
}

impl CanGetTypee for RegexWord {
    fn get_type(&self) -> String {
        "urn:api:regexword:regexword".to_string()
    }
}

impl From<RegexWord> for RegexWordView {
    fn from(value: RegexWord) -> Self {
        RegexWordView {
            data: value.data.into(),
        }
    }
}

impl From<RegexWordData> for RegexWordDataView {
    fn from(value: RegexWordData) -> Self {
        Self {
            regex_parts: value.regex_parts,
            word: value.word,
            order: value.order,
            nb_selected: value.nb_selected,
            date_last_selected: value.date_last_selected,
        }
    }
}

