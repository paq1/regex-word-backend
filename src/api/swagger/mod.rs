use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

use crate::api::regexword::routes::exemple_wit_api_key_routes::__path_exemple_api_key;
use crate::api::regexword::routes::read_routes::__path_fetch_bandeau_events;
use crate::api::regexword::routes::read_routes::__path_fetch_many_bandeau;
use crate::api::regexword::routes::read_routes::__path_fetch_one_bandeau;
use crate::api::regexword::routes::write_routes::__path_disable_one_bandeau;
use crate::api::regexword::routes::write_routes::__path_prod_one_bandeau;
use crate::api::regexword::routes::write_routes::__path_insert_one_bandeau;
use crate::api::regexword::routes::write_routes::__path_update_one_bandeau;
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
        fetch_many_bandeau,
        fetch_one_bandeau,
        insert_one_bandeau,
        update_one_bandeau,
        disable_one_bandeau,
        prod_one_bandeau,
        fetch_bandeau_events,
        exemple_api_key,
    ),
    components(
        schemas(
            CreateApiKeyCommand,
            CreateBandeauCommand,
            BandeauCreatedView,
            ManyView < BandeauViewState >,
            CreateBandeauCommand,
            UpdateBandeauCommand,
            DisableBandeauCommand,
            ProductionBandeauCommand,
            DataWrapperView < ApiView < BandeauViewEvent > >,
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