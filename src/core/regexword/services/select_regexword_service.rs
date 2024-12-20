use std::sync::Arc;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use framework_cqrs_lib::cqrs::models::errors::{Error, ErrorHttpCustom, ResultErr};
use log::{error, info};
use crate::core::regexword::command_handler::select_handler::RegexWordSelecteOneHandler;
use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::states::RegexWordStates;
use crate::core::regexword::repositories::CustomRegexWordRepository;
use crate::models::regexword::commands::{RegexWordCommands, SelectOneRegexWordCommand};


// TODO : implémentez ca dans la struct ()

#[async_trait]
pub trait SelectRegexWordService: Send + Sync {
    /// WARN: this service mustn't be injected in command handler (cyclic dependency)

    async fn valid_current_word(&self, word: &str, ctx: &Context) -> ResultErr<(bool, Vec<u32>)> {
        self.get_current_or_select_one(ctx)
            .await
            .and_then(|entity| self.validate_current_word(word, &entity))
    }

    // TODO : bouger ca dans l'impl du trait (devrait etre privé)
    fn validate_current_word(&self, word: &str, entity: &Entity<RegexWordStates, String>) -> ResultErr<(bool, Vec<u32>)> {
        let correspondance_index = word.chars()
            .enumerate()
            .flat_map(|(index, current_letter)| {
                let expected_letter = entity.data.get_word().chars().collect::<Vec<_>>().get(index).map(|c| c.clone());
                expected_letter
                    .and_then(|expected_letter| if format!("{expected_letter}").to_lowercase() == format!("{current_letter}").to_lowercase() { Some(vec![index]) } else { None })
            })
            .flatten()
            .map(|x| x as u32)
            .collect::<Vec<_>>();

        Ok((correspondance_index.len() == word.len(), correspondance_index))
    }

    async fn get_current_or_select_one(&self, ctx: &Context) -> ResultErr<Entity<RegexWordStates, String>> {
        let regex_already_exist = self.get_repository().exists_at_date(&ctx.now.date_naive()).await?;
        if regex_already_exist {
            self.get_repository()
                .fetch_word_from_date(&ctx.now.date_naive()).await?
                .ok_or({
                        Error::Http(
                            ErrorHttpCustom {
                                title: "pas de regex trouvé".to_string(),
                                code: "76PDMTA".to_string(),
                                causes: vec![],
                                status: Some(500u16),
                            }
                        )
                    }
                )
        } else {
            info!("selection du nouveau mot du jour");
            self.unsafe_select_regex(ctx).await
        }
    }

    async fn unsafe_select_regex(&self, ctx: &Context) -> ResultErr<Entity<RegexWordStates, String>> {
        let mot = self.get_repository()
            .fetch_one_word_random(&ctx.now.date_naive()).await
            .and_then(|data| {
                if data.is_none() {
                    error!("pas de mot trouvé");
                }

                data
                    .ok_or(
                        {
                            Error::Http(
                                ErrorHttpCustom {
                                    title: "pas de mot trouvé".to_string(),
                                    code: "76PDMTA".to_string(),
                                    causes: vec![],
                                    status: Some(500u16),
                                }
                            )
                        }
                    )
            })?;

        let result = self.engine_regex_word()
            .compute(
                RegexWordCommands::SelectOne(SelectOneRegexWordCommand { reset_regex: Some(true) }),
                mot.entity_id,
                RegexWordSelecteOneHandler::handler_name().to_string(),
                &ctx
            ).await?;

        Ok(result.1)
    }


    /// services dependencies
    fn get_repository(&self) -> Arc<dyn CustomRegexWordRepository>;
    fn engine_regex_word(&self) -> Arc<Engine<RegexWordStates, RegexWordCommands, RegexWordEvents>>;
}
