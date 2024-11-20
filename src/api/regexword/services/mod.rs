pub mod rules;

use async_trait::async_trait;
use crate::core::regexword::services::RegexWordService;

pub struct RegexWordServiceImpl {}

#[async_trait]
impl RegexWordService for RegexWordServiceImpl {}
