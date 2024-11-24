pub mod rules;
pub mod select_regexword_service;
pub mod random;
pub mod generate_regex;

use async_trait::async_trait;

#[async_trait]
pub trait RegexWordService: Send + Sync {

}
