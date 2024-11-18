use async_trait::async_trait;

use crate::core::regexword::data::events::{RegexWordDisabled, RegexWordEvents};
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::commands::RegexWordCommands;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerUpdate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};

pub struct RegexWordDisableHandler;
#[async_trait]
impl CommandHandlerUpdate<RegexWordStates, RegexWordCommands, RegexWordEvents> for RegexWordDisableHandler {
    fn name(&self) -> String {
        Self::handler_name().to_string()
    }

    async fn on_command(&self, _id: String, _state: RegexWordStates, command: RegexWordCommands, context: &Context) -> ResultErr<RegexWordEvents> {
        match command {
            RegexWordCommands::Disable(_) => Ok(
                RegexWordEvents::Disabled(RegexWordDisabled { by: context.subject.clone(), at: context.now  })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}
impl RegexWordDisableHandler {
    pub fn handler_name() -> &'static str {
        "disable-regexword"
    }
}
