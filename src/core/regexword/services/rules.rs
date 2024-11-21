use std::sync::Arc;
use async_trait::async_trait;
use chrono::NaiveDate;
use framework_cqrs_lib::cqrs::models::errors::{Error, ErrorHttpCustom, Problem, ResultErr};
use crate::core::regexword::repositories::CustomRegexWordRepository;

#[async_trait]
pub trait Rules: Sync + Send {
    async fn can_select(&self, date: &NaiveDate) -> ResultErr<bool> {
        self.get_repository().exists_at_date(date).await
            .map(|exist| !exist)
    }

    async fn can_insert(&self, word: &str) -> ResultErr<()> {
        let word_already_exist = self.get_repository().exists_word(word).await?;

        let rules_with_causes = vec![
            (word.len() < 3, "Mot trop court, doit faire au moins 3 caractères"),
            (!word.to_lowercase().chars().all(|c| c.is_ascii_lowercase()), "Certain caratère ne sont pas dans [a-z]"),
            (word_already_exist, "Le mot existe déjà dans le référentiel.")
        ];

        let invalid_rules = rules_with_causes
            .into_iter()
            .filter(|(is_invalid, _)| *is_invalid)
            .collect::<Vec<_>>();

        if invalid_rules.is_empty() {
            Ok(())
        } else {

            let problems = invalid_rules
                .into_iter()
                .map(|(_, message)| Problem {
                    title: message.to_string(), description: "bad request".to_string()
                })
                .collect::<Vec<_>>();

            Err(Error::Http(ErrorHttpCustom {
                title: "bad request".to_string(),
                code: "76ECHRW".to_string(),
                causes: problems,
                status: Some(400u16),
            }))
        }

    }

    fn get_repository(&self) -> Arc<dyn CustomRegexWordRepository>;
}