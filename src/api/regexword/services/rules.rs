use std::sync::Arc;
use crate::core::regexword::repositories::CustomRegexWordRepository;
use crate::core::regexword::services::rules::Rules;

pub struct RulesImpl {
    pub store: Arc<dyn CustomRegexWordRepository>
}

impl Rules for RulesImpl {
    fn get_repository(&self) -> Arc<dyn CustomRegexWordRepository> {
        self.store.clone()
    }
}