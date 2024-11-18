use crate::core::regexword::data::events::{RegexWordActivated, RegexWordEvents};
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::commands::RegexWordCommands;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerUpdate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};

pub struct RegexWordActivateHandler {}
#[async_trait]
impl CommandHandlerUpdate<RegexWordStates, RegexWordCommands, RegexWordEvents> for RegexWordActivateHandler {
    fn name(&self) -> String {
        Self::handler_name().to_string()
    }

    async fn on_command(&self, _id: String, _state: RegexWordStates, command: RegexWordCommands, context: &Context) -> ResultErr<RegexWordEvents> {
        match command {
            RegexWordCommands::Activate(_) => Ok(
                RegexWordEvents::Activated(RegexWordActivated {
                    by: context.subject.clone(),
                    at: context.now,
                    date_activate: context.now.date_naive()
                })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

impl RegexWordActivateHandler {
    pub fn handler_name() -> &'static str {
        "activate-regexword"
    }
}