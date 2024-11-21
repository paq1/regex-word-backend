use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::states::RegexWordStates;
use crate::core::regexword::repositories::CustomRegexWordRepository;
use crate::core::regexword::services::select_regexword_service::SelectRegexWordService;
use crate::models::regexword::commands::RegexWordCommands;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use std::sync::Arc;

pub struct SelectRegexWordServiceImpl {
    pub store: Arc<dyn CustomRegexWordRepository>,
    pub engine: Arc<Engine<RegexWordStates, RegexWordCommands, RegexWordEvents>>
}

#[async_trait]
impl SelectRegexWordService for SelectRegexWordServiceImpl {
    fn get_repository(&self) -> Arc<dyn CustomRegexWordRepository> {
        self.store.clone()
    }

    fn engine_regex_word(&self) -> Arc<Engine<RegexWordStates, RegexWordCommands, RegexWordEvents>> {
        self.engine.clone()
    }
}