use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::events::RegexWordEvents::Created;
use crate::core::regexword::data::states::regexword_activate::RegexWordActivate;
use crate::core::regexword::data::states::regexword_disable::RegexWordDisable;
use crate::models::regexword::views::RegexWordViewState;
use framework_cqrs_lib::cqrs::models::jsonapi::{CanBeView, CanGetTypee};

pub mod regexword_activate;
pub mod regexword_disable;

#[derive(Clone, Debug)]
pub enum RegexWordStates {
    RegexWordActivate(RegexWordActivate),
    RegexWordDisable(RegexWordDisable),
}

impl RegexWordStates {
    pub fn reduce_state(&self, event: RegexWordEvents) -> Option<RegexWordStates> {
        match self {
            RegexWordStates::RegexWordActivate(c) => c.reduce_state(event),
            RegexWordStates::RegexWordDisable(c) => c.reduce_state(event),
        }
    }

    pub fn reduce_state_from_empty(event: RegexWordEvents) -> Option<RegexWordStates> {
        match event {
            Created(data) =>
                Some(
                    RegexWordStates::RegexWordDisable(
                        RegexWordDisable {
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
            RegexWordStates::RegexWordActivate(state) => state.get_type(),
            RegexWordStates::RegexWordDisable(state) => state.get_type(),
        }
    }
}

impl CanBeView<RegexWordViewState> for RegexWordStates {
    fn to_view(&self) -> RegexWordViewState {
        match self.clone() {
            RegexWordStates::RegexWordActivate(state) =>
                RegexWordViewState::Activate(state.into()),
            RegexWordStates::RegexWordDisable(state) =>
                RegexWordViewState::Disable(state.into()),
        }
    }
}