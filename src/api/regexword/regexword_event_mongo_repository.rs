use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::api::regexword::regexword_dbo::RegexWordDboEvent;
use crate::api::regexword::mappers::events::{from_regexword_event_dbo_to_event, from_regexword_event_to_dbo};
use crate::core::regexword::data::events::RegexWordEvents;
use framework_cqrs_lib::cqrs::core::daos::DAO;
use framework_cqrs_lib::cqrs::core::data::EntityEvent;
use framework_cqrs_lib::cqrs::core::repositories::can_fetch_all::CanFetchAll;
use framework_cqrs_lib::cqrs::core::repositories::events::{ReadOnlyEventRepo, RepositoryEvents, WriteOnlyEventRepo};
use framework_cqrs_lib::cqrs::core::repositories::query::Query;
use framework_cqrs_lib::cqrs::core::repositories::CanFetchMany;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EventDBO;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;

pub struct RegexWordEventMongoRepository {
    pub dao: Arc<Mutex<dyn DAO<EventDBO<RegexWordDboEvent, String>, String>>>,
}

#[async_trait]
impl RepositoryEvents<RegexWordEvents, String> for RegexWordEventMongoRepository {}

#[async_trait]
impl CanFetchAll<EntityEvent<RegexWordEvents, String>> for RegexWordEventMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<EntityEvent<RegexWordEvents, String>>> {
        self.dao
            .lock().await
            .fetch_all(query)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(|dbo| from_regexword_event_dbo_to_event(dbo))
                    .collect()
            })
    }
}

#[async_trait]
impl CanFetchMany<EntityEvent<RegexWordEvents, String>> for RegexWordEventMongoRepository {}

#[async_trait]
impl ReadOnlyEventRepo<RegexWordEvents, String> for RegexWordEventMongoRepository {
    async fn fetch_one(&self, event_id: &String) -> ResultErr<Option<EntityEvent<RegexWordEvents, String>>> {
        self.dao.lock().await.fetch_one(event_id).await.map(|maybevent| {
            maybevent.map(|event_dbo| {
                from_regexword_event_dbo_to_event(event_dbo)
            })
        })
    }
}

#[async_trait]
impl WriteOnlyEventRepo<RegexWordEvents, String> for RegexWordEventMongoRepository {
    async fn insert(&self, entity_event: &EntityEvent<RegexWordEvents, String>) -> ResultErr<String> {
        let dao: EventDBO<RegexWordDboEvent, String> = from_regexword_event_to_dbo(entity_event.clone());

        let dao_sanitize_version: EventDBO<RegexWordDboEvent, String> = EventDBO {
            version: Some(0),
            ..dao.clone()
        };

        self.dao.lock().await.insert(&dao_sanitize_version, &dao.entity_id).await
    }
}