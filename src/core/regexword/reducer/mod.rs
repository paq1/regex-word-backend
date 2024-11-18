use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::states::RegexWordStates;
use framework_cqrs_lib::cqrs::core::reducer::Reducer;

pub struct RegexWordReducer {
    pub underlying: Reducer<RegexWordEvents, RegexWordStates>,
}

impl RegexWordReducer {
    pub fn new() -> Self {
        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    match current {
                        Some(current_state) => current_state.reduce_state(event),
                        None => RegexWordStates::reduce_state_from_empty(event)
                    }
                }
            }
        }
    }
}