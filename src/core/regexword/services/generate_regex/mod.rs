use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};
use regex_generator::core::services::regex_generator::RegexGenerator;


pub struct GenerateRegexService {}

impl CanGenerateRegex for GenerateRegexService {}

pub trait CanGenerateRegex: Send + Sync {
    fn generate_regex(&self, word: &str, total: u32) -> ResultErr<Vec<String>> {

        let _ = if total > word.len() as u32 {
            Err(Error::Simple("nb regex > word length".to_string()))
        } else { Ok(()) }?;

        let regexes = RegexGenerator::regexes_splited(word, Some(word.len() as u32 / total)).into_iter().map(|rw| rw.regex).collect::<Vec<_>>();

        let sanitized_regex_parts = if regexes.len() > 3 {
            let heads = regexes.clone().into_iter().take(2).collect::<Vec<_>>();
            let lasts = regexes[2..].into_iter().map(|chaine| chaine.clone()).collect::<Vec<_>>().join("");
            let lasts_vec = vec![lasts];
            [&heads[..], &lasts_vec[..]].concat()
        } else {
            regexes
        };

        Ok(sanitized_regex_parts)
    }
}

