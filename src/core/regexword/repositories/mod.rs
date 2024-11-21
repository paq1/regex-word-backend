use crate::core::regexword::data::states::RegexWordStates;
use async_trait::async_trait;
use chrono::NaiveDate;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::core::repositories::entities::RepositoryEntity;
use framework_cqrs_lib::cqrs::core::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use framework_cqrs_lib::cqrs::core::repositories::query::{PaginationDef, Query};
use framework_cqrs_lib::cqrs::models::errors::ResultErr;
use log::{info, warn};

#[async_trait]
pub trait CustomRegexWordRepository: RepositoryEntity<RegexWordStates, String> {
    // TODO : ajouter les queries specifiques a l'ontology regexword

    async fn fetch_one_word_random(&self, date: &NaiveDate) -> ResultErr<Option<Entity<RegexWordStates, String>>>;

    async fn exists_at_date(&self, date: &NaiveDate) -> ResultErr<bool> {
        self.fetch_word_from_date(date)
            .await
            .map(|r| r.is_some())
    }

    async fn fetch_word_from_date(&self, date: &NaiveDate) -> ResultErr<Option<Entity<RegexWordStates, String>>> {
        info!("exists_at_date: {}", date.to_string());

        self
            .fetch_all(Query {
                pagination: PaginationDef::default(),
                filter: Filter::Expr(Expr::ExprStr(ExprGeneric {
                    field: "data.date_last_selected".to_string(),
                    operation: Operation::EqualsTo,
                    head: date.to_string(),
                })),
            })
            .await
            .map(|entities| {
                if entities.len() > 1 {
                    warn!("found more that one entity for the same date_last_selected, this not possible for from the business logic");
                    warn!("nb element: {} for {}", entities.len(), date.to_string());
                };
                entities.clone().first().map(|r| r.clone())
            })
    }
}
