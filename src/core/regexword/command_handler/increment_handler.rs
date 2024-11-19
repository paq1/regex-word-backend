use crate::core::regexword::data::events::{RegexWordIncremented, RegexWordEvents};
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::commands::RegexWordCommands;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerUpdate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};

pub struct RegexWordIncrementNbSelectedHandler {}
#[async_trait]
impl CommandHandlerUpdate<RegexWordStates, RegexWordCommands, RegexWordEvents> for RegexWordIncrementNbSelectedHandler {
    fn name(&self) -> String {
        Self::handler_name().to_string()
    }

    async fn on_command(&self, _id: String, _state: RegexWordStates, command: RegexWordCommands, context: &Context) -> ResultErr<RegexWordEvents> {
        match command {
            RegexWordCommands::Increment(_) => Ok(
                RegexWordEvents::Incremented(RegexWordIncremented {
                    by: context.subject.clone(),
                    at: context.now,
                    nb_selected: _state.get_nb_selected() + 1
                })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

impl RegexWordIncrementNbSelectedHandler {
    pub fn handler_name() -> &'static str {
        "increment-regexword"
    }
}