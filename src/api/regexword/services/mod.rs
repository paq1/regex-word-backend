pub mod rules;
pub mod select_regexword_service_impl;
pub mod random_order_generator_impl;

use crate::core::regexword::services::RegexWordService;
use async_trait::async_trait;

pub struct RegexWordServiceImpl {}

#[async_trait]
impl RegexWordService for RegexWordServiceImpl {}
