use crate::core::regexword::data::events::{RegexWordEvents, RegexWordSelected};
use crate::core::regexword::data::states::RegexWordStates;
use crate::core::regexword::services::rules::Rules;
use crate::models::regexword::commands::RegexWordCommands;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerUpdate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ErrorHttpCustom, ResultErr};
use std::sync::Arc;

pub struct RegexWordSelecteOneHandler {
    pub rules: Arc<dyn Rules>,
}
#[async_trait]
impl CommandHandlerUpdate<RegexWordStates, RegexWordCommands, RegexWordEvents> for RegexWordSelecteOneHandler {
    fn name(&self) -> String {
        Self::handler_name().to_string()
    }

    async fn on_command(&self, _id: String, _state: RegexWordStates, command: RegexWordCommands, context: &Context) -> ResultErr<RegexWordEvents> {
        match command {
            RegexWordCommands::SelectOne(_) =>
                self.rules.can_select(&context.now.date_naive()).await
                    .and_then(|can_selected| {
                        if can_selected {
                            Ok(
                                RegexWordEvents::Selected(RegexWordSelected {
                                    by: context.subject.clone(),
                                    at: context.now.clone(),
                                    nb_selected: _state.get_nb_selected() + 1,
                                    date_last_selected: context.now.date_naive(),
                                })
                            )
                        } else {
                            Err(Error::Http(ErrorHttpCustom {
                                title: "Une regex est déjà selectionné pour aujourd'hui".to_string(),
                                code: "76dejas".to_string(),
                                causes: vec![],
                                status: Some(400u16),
                            }))
                        }
                    }),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

impl RegexWordSelecteOneHandler {
    pub fn handler_name() -> &'static str {
        "select-one-regexword"
    }
}