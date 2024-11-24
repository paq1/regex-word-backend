pub mod rules;
pub mod select_regexword_service;
pub mod random_order_generator;

use async_trait::async_trait;

#[async_trait]
pub trait RegexWordService: Send + Sync {

}
