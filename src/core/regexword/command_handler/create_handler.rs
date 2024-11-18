use async_trait::async_trait;

use crate::core::regexword::data::events::{RegexWordCreated, RegexWordDisabled, RegexWordEvents};
use crate::core::regexword::data::regexword_data::RegexWordData;
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::commands::RegexWordCommands;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerCreate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};

pub struct RegexWordCreateHandler {}
#[async_trait]
impl CommandHandlerCreate<RegexWordStates, RegexWordCommands, RegexWordEvents> for RegexWordCreateHandler {
    fn name(&self) -> String {
        Self::handler_name().to_string()
    }

    async fn on_command(&self, _id: String, command: RegexWordCommands, context: &Context) -> ResultErr<RegexWordEvents> {
        match command {
            RegexWordCommands::Create(c) => {
                Ok(
                    RegexWordEvents::Created(
                        RegexWordCreated {
                            by: context.clone().subject,
                            at: context.clone().now,
                            data: RegexWordData {
                                word: c.word,
                                regex_parts: c.regex_parts,
                                niveau_difficulte: c.niveau_difficulte,
                            },
                        }
                    )
                )
            }
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

impl RegexWordCreateHandler {
    pub fn handler_name() -> &'static str {
        "create-regexword"
    }
}
