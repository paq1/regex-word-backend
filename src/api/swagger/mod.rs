use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

use crate::api::regexword::routes::exemple_wit_api_key_routes::__path_exemple_api_key;
use crate::api::regexword::routes::read_routes::__path_fetch_regexword_events;
use crate::api::regexword::routes::read_routes::__path_fetch_many_regexword;
use crate::api::regexword::routes::read_routes::__path_dayword_regexword;
use crate::api::regexword::routes::read_routes::__path_fetch_one_regexword;
use crate::api::regexword::routes::write_routes::__path_insert_one_regexword;
use crate::api::regexword::routes::write_routes::__path_select_one_regexword;
use crate::models::regexword::commands::*;
use crate::models::regexword::views::*;
use framework_cqrs_lib::cqrs::core::repositories::query::{InfoPaged, Page};
use framework_cqrs_lib::cqrs::models::api_key::commands::CreateApiKeyCommand;
use framework_cqrs_lib::cqrs::models::jsonapi::ManyView;
use framework_cqrs_lib::cqrs::models::views::command_handler_view::ApiView;
use framework_cqrs_lib::cqrs::models::views::DataWrapperView;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        dayword_regexword,
        fetch_many_regexword,
        fetch_one_regexword,
        insert_one_regexword,
        select_one_regexword,
        fetch_regexword_events,
        exemple_api_key,
    ),
    components(
        schemas(
            CreateApiKeyCommand,
            CreateRegexWordCommand,
            RegexWordCreatedView,
            ManyView < RegexWordViewState >,
            CreateRegexWordCommand,
            SelectOneRegexWordCommand,
            DataWrapperView < ApiView < RegexWordViewEvent > >,
            InfoPaged,
            Page
        )
    ),
    modifiers(& SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {

        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()
            ),
        );
        components.add_security_scheme(
            "api_key_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-KEY"))),
        )
    }
}