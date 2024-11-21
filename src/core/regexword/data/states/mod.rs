use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::events::RegexWordEvents::Created;
use crate::core::regexword::data::states::regexword::RegexWord;
use crate::models::regexword::views::RegexWordViewState;
use framework_cqrs_lib::cqrs::models::jsonapi::{CanBeView, CanGetTypee};

pub mod regexword;

#[derive(Clone, Debug)]
pub enum RegexWordStates {
    RegexWord(RegexWord),
}

impl RegexWordStates {
    pub fn reduce_state(&self, event: RegexWordEvents) -> Option<RegexWordStates> {
        match self {
            RegexWordStates::RegexWord(c) => c.reduce_state(event),
        }
    }

    pub fn get_word(&self) -> String {
        match self {
            RegexWordStates::RegexWord(regexword) => regexword.data.word.clone()
        }
    }

    pub fn get_nb_selected(&self) -> u32 {
        match self {
            RegexWordStates::RegexWord(regexword) => regexword.data.nb_selected
        }
    }

    pub fn get_regex_parts(&self) -> Vec<String> {
        match self {
            RegexWordStates::RegexWord(regexword) => regexword.data.regex_parts.clone()
        }
    }

    pub fn reduce_state_from_empty(event: RegexWordEvents) -> Option<RegexWordStates> {
        match event {
            Created(data) =>
                Some(
                    RegexWordStates::RegexWord(
                        RegexWord {
                            kind: "urn:api:regexword:regexword".to_string(),
                            data: data.data,
                        }
                    )
                ),
            _ => None
        }
    }
}

impl CanGetTypee for RegexWordStates {
    fn get_type(&self) -> String {
        match self {
            RegexWordStates::RegexWord(state) => state.get_type(),
        }
    }
}

impl CanBeView<RegexWordViewState> for RegexWordStates {
    fn to_view(&self) -> RegexWordViewState {
        match self.clone() {
            RegexWordStates::RegexWord(state) =>
                RegexWordViewState::Activate(state.into()),
        }
    }
}