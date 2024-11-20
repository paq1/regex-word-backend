use std::sync::Arc;
use async_trait::async_trait;
use chrono::NaiveDate;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;
use crate::core::regexword::repositories::CustomRegexWordRepository;

#[async_trait]
pub trait Rules: Sync + Send {
    async fn can_select(&self, date: &NaiveDate) -> ResultErr<bool> {
        self.get_repository().exists_at_date(date).await
            .map(|exist| !exist)
    }

    fn get_repository(&self) -> Arc<dyn CustomRegexWordRepository>;
}