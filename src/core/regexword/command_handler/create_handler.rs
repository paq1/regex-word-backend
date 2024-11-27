use async_trait::async_trait;
use std::sync::Arc;

use crate::core::regexword::data::events::{RegexWordCreated, RegexWordEvents};
use crate::core::regexword::data::regexword_data::RegexWordData;
use crate::core::regexword::data::states::RegexWordStates;
use crate::core::regexword::services::rules::Rules;
use crate::models::regexword::commands::RegexWordCommands;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerCreate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ErrorHttpCustom, ResultErr};
use crate::core::regexword::services::generate_regex::CanGenerateRegex;
use crate::core::regexword::services::random::CanGenerateRandomOrder;

pub struct RegexWordCreateHandler {
    pub rules: Arc<dyn Rules>,
    pub random_order_generator_service: Arc<dyn CanGenerateRandomOrder>,
    pub generate_regex_service: Arc<dyn CanGenerateRegex>,
}
#[async_trait]
impl CommandHandlerCreate<RegexWordStates, RegexWordCommands, RegexWordEvents> for RegexWordCreateHandler {
    fn name(&self) -> String {
        Self::handler_name().to_string()
    }

    async fn on_command(&self, _id: String, command: RegexWordCommands, context: &Context) -> ResultErr<RegexWordEvents> {
        match command {
            RegexWordCommands::Create(c) => {
                let _ = self.rules.can_insert(c.word.as_str()).await?;

                let regex_parts = self.generate_regex_service.generate_regex(&c.word, 3)?;

                let order = self.random_order_generator_service.generate_random_selected_order()?;

                if regex_parts.len() != 3 {
                    Err(Error::Http(ErrorHttpCustom::new("Erreur lors de la génération des regexes", "78failr", vec![], Some(500u16))))
                } else {
                    Ok(
                        RegexWordEvents::Created(
                            RegexWordCreated {
                                by: context.clone().subject,
                                at: context.clone().now,
                                data: RegexWordData {
                                    word: c.word.to_lowercase(),
                                    nb_selected: 0,
                                    date_last_selected: None,
                                    regex_parts,
                                    order,
                                },
                            }
                        )
                    )
                }
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
