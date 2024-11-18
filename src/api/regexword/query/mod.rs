use actix_web::web::Query;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use framework_cqrs_lib::cqrs::core::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use framework_cqrs_lib::cqrs::core::repositories::query::PaginationDef;

#[derive(Serialize, Deserialize, IntoParams, Debug, Clone)]
pub struct FetchManyBandeauQuery {
    #[serde(rename = "page[number]")]
    pub number: Option<usize>,
    #[serde(rename = "page[size]")]
    pub size: Option<usize>,
    #[serde(rename = "isProd")]
    pub is_prod: Option<bool>,
}

pub fn from_fetch_many_bandeau_query_to_query_repo(q: Query<FetchManyBandeauQuery>) -> framework_cqrs_lib::cqrs::core::repositories::query::Query {
    let size = q.size.unwrap_or(10);
    let number = q.number.unwrap_or(0);
    let is_prod = q.is_prod.unwrap_or(false);

    framework_cqrs_lib::cqrs::core::repositories::query::Query {
        pagination: PaginationDef {
            page_number: number,
            page_size: size,
        },
        filter: {
            if is_prod {
                Filter::Expr(Expr::ExprStr(ExprGeneric {
                    field: "data.type".to_string(),
                    operation: Operation::EqualsTo,
                    head: "BandeauProdDbo".to_string()
                }))
            } else {
                Filter::None
            }
        },
    }
}


#[derive(Serialize, Deserialize, IntoParams, Debug, Clone)]
pub struct BandeauQuery {
    #[serde(rename = "page[number]")]
    pub number: Option<usize>,
    #[serde(rename = "page[size]")]
    pub size: Option<usize>,
}

pub fn from_bandeau_query_to_query_repo(q: Query<BandeauQuery>) -> framework_cqrs_lib::cqrs::core::repositories::query::Query {
    let size = q.size.unwrap_or(10);
    let number = q.number.unwrap_or(0);

    framework_cqrs_lib::cqrs::core::repositories::query::Query {
        pagination: PaginationDef {
            page_number: number,
            page_size: size,
        },
        filter: Filter::None,
    }
}