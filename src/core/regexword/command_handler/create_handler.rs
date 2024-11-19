use async_trait::async_trait;

use crate::core::regexword::data::events::{RegexWordCreated, RegexWordEvents};
use crate::core::regexword::data::regexword_data::RegexWordData;
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::commands::RegexWordCommands;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerCreate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ErrorHttpCustom, ResultErr};
use regex_generator::core::services::regex_generator::RegexGenerator;

pub struct RegexWordCreateHandler {}
#[async_trait]
impl CommandHandlerCreate<RegexWordStates, RegexWordCommands, RegexWordEvents> for RegexWordCreateHandler {
    fn name(&self) -> String {
        Self::handler_name().to_string()
    }

    async fn on_command(&self, _id: String, command: RegexWordCommands, context: &Context) -> ResultErr<RegexWordEvents> {
        match command {
            RegexWordCommands::Create(c) => {

                if c.word.len() < 3 {
                    Err(Error::Http(ErrorHttpCustom::new("Mot trop court", "78small", vec![], Some(400u16))))
                } else {
                    let regexes = RegexGenerator::regexes_splited(&c.word, Some((c.word.len() / 3) as u32)).into_iter().map(|rw| rw.regex).collect::<Vec<_>>();

                    let sanitized_regex_parts = if regexes.len() > 3 {
                        let heads = regexes.clone().into_iter().take(2).collect::<Vec<_>>();
                        let lasts = regexes[2..].into_iter().map(|chaine| chaine.clone()).collect::<Vec<_>>().join("");
                        let lasts_vec = vec![lasts];
                        [&heads[..], &lasts_vec[..]].concat()
                    } else {
                        regexes
                    };

                    if sanitized_regex_parts.len() != 3 {
                        Err(Error::Http(ErrorHttpCustom::new("Erreur lors de la génération des regexes", "78failr", vec![], Some(500u16))))
                    } else {
                        Ok(
                            RegexWordEvents::Created(
                                RegexWordCreated {
                                    by: context.clone().subject,
                                    at: context.clone().now,
                                    data: RegexWordData {
                                        word: c.word,
                                        regex_parts: sanitized_regex_parts,
                                        niveau_difficulte: "simple".to_string(), // TODO : supprimer
                                        nb_selected: 0,
                                    },
                                }
                            )
                        )
                    }
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
