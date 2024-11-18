use crate::core::regexword::services::RegexWordService;
use async_trait::async_trait;

pub struct BandeauServiceImpl {}

#[async_trait]
impl RegexWordService for BandeauServiceImpl {}
