use crate::api::regexword::services::RegexWordServiceImpl;
use crate::api::regexword::regexword_dbo::{RegexWordDboEvent, RegexWordDboState};
use crate::api::regexword::regexword_event_mongo_repository::RegexWordEventMongoRepository;
use crate::api::regexword::regexword_mongo_dao::{RegexWordEventMongoDAO, RegexWordMongoDAO};
use crate::api::regexword::regexword_mongo_repository::MongoRegexWordRepository;
use crate::core::regexword::command_handler::create_handler::RegexWordCreateHandler;
use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::states::RegexWordStates;
use crate::core::regexword::reducer::RegexWordReducer;
use crate::core::regexword::repositories::CustomRegexWordRepository;
use crate::core::regexword::services::RegexWordService;
use crate::models::regexword::commands::RegexWordCommands;
use framework_cqrs_lib::cqrs::core::daos::DAO;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandler;
use framework_cqrs_lib::cqrs::core::repositories::events::RepositoryEvents;
use framework_cqrs_lib::cqrs::infra::authentication::AuthenticationComponent;
use framework_cqrs_lib::cqrs::infra::daos::dbos::{EntityDBO, EventDBO};
use futures::lock::Mutex;
use std::sync::Arc;
use crate::core::regexword::command_handler::increment_handler::RegexWordIncrementNbSelectedHandler;

pub struct RegexWordComponent {
    pub store: Arc<dyn CustomRegexWordRepository>,
    pub journal: Arc<dyn RepositoryEvents<RegexWordEvents, String>>,
    pub service: Arc<dyn RegexWordService>,
    pub engine: Arc<Engine<RegexWordStates, RegexWordCommands, RegexWordEvents>>,
}

impl RegexWordComponent {
    pub async fn new(_authentication_component: &Arc<AuthenticationComponent>) -> Self {
        let dbname = "regexwordapi";

        let dao_store: Arc<Mutex<dyn DAO<EntityDBO<RegexWordDboState, String>, String>>> =
            Arc::new(Mutex::new(RegexWordMongoDAO::new(dbname, "regexword_store").await));
        let dao_journal: Arc<Mutex<dyn DAO<EventDBO<RegexWordDboEvent, String>, String>>> =
            Arc::new(Mutex::new(RegexWordEventMongoDAO::new(dbname, "regexword_journal").await));

        // repo
        let store = Arc::new(
            MongoRegexWordRepository {
                dao: Arc::clone(&dao_store)
            }
        );

        let journal: Arc<dyn RepositoryEvents<RegexWordEvents, String>> = Arc::new(
            RegexWordEventMongoRepository {
                dao: Arc::clone(&dao_journal)
            }
        );

        // services
        let service: Arc<dyn RegexWordService> = Arc::new(
            RegexWordServiceImpl {}
        );

        let engine: Arc<Engine<RegexWordStates, RegexWordCommands, RegexWordEvents>> = Arc::new(Engine {
            handlers: vec![
                CommandHandler::Create(
                    Box::new(
                        RegexWordCreateHandler {}
                    )
                ),
                CommandHandler::Update(Box::new(RegexWordIncrementNbSelectedHandler {})),
            ],
            reducer: RegexWordReducer::new().underlying,
            store: store.clone(),
            journal: journal.clone(),
        });

        Self {
            store,
            journal,
            service,
            engine,
        }
    }
}