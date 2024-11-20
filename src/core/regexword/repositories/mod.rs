use crate::core::regexword::data::states::RegexWordStates;
use async_trait::async_trait;
use chrono::NaiveDate;
use framework_cqrs_lib::cqrs::core::repositories::entities::RepositoryEntity;
use framework_cqrs_lib::cqrs::core::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use framework_cqrs_lib::cqrs::core::repositories::query::{PaginationDef, Query};
use framework_cqrs_lib::cqrs::models::errors::ResultErr;
use log::info;

#[async_trait]
pub trait CustomRegexWordRepository: RepositoryEntity<RegexWordStates, String> {
    // TODO : ajouter les queries specifiques a l'ontology regexword
    async fn exists_at_date(&self, date: &NaiveDate) -> ResultErr<bool> {

        info!("exists_at_date: {}", date.to_string());

        self
            .fetch_all(Query {
                pagination: PaginationDef::default(),
                filter: Filter::Expr(Expr::ExprStr(ExprGeneric {
                    field: "data.date_last_selected".to_string(),
                    operation: Operation::EqualsTo,
                    head: date.to_string()
                }))
            })
            .await
            .map(|data| {
                info!("nb element: {}", data.len());
                !data.is_empty()
            })
    }
}
