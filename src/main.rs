use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use framework_cqrs_lib::cqrs::infra::api_key::component::ApiKeyComponent;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::regexword::routes::read_routes::{fetch_many_regexword, fetch_one_regexword};
use api::regexword::routes::write_routes::{insert_one_regexword, activate_one_regexword};


use crate::api::regexword::regexword_component::RegexWordComponent;
use crate::api::regexword::routes::exemple_wit_api_key_routes::exemple_api_key;
use crate::api::regexword::routes::read_routes::{fetch_regexword_events, fetch_one_regexword_event};
use crate::api::regexword::routes::write_routes::{disable_one_regexword};
use crate::api::swagger::ApiDoc;
use framework_cqrs_lib::cqrs::infra::authentication::AuthenticationComponent;


use framework_cqrs_lib::cqrs::models::errors::StandardHttpError;
use log::info;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("lancement du server");

    let authentication_component = Arc::new(AuthenticationComponent::new().unwrap());
    let api_key_component = Arc::new(ApiKeyComponent::new(
        "regexwordapi", "regexword",
    ).await);

    // regexword aggregat
    let regexword_component = RegexWordComponent::new(&authentication_component.clone()).await;

    let openapi = ApiDoc::openapi();
    let api_address = std::env::var("API_ADDRESS").unwrap();
    let api_port = std::env::var("API_PORT").unwrap().parse::<u16>().unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .supports_credentials();

        let standard_http_error = StandardHttpError::new();

        App::new()
            .wrap(cors)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
            .app_data(web::Data::new(standard_http_error))
            .app_data(web::Data::new(authentication_component.jwt_token_decoder_service.clone()))
            // regexword services
            .service(
                web::scope("/regexword")
                    .service(fetch_one_regexword)
                    .service(fetch_one_regexword_event)
                    .service(fetch_many_regexword)
                    .service(fetch_regexword_events)
                    .service(insert_one_regexword)
                    .service(activate_one_regexword)
                    .service(disable_one_regexword)
                    .service(exemple_api_key)
                    .app_data(web::Data::new(Arc::clone(&regexword_component.engine)))
                    .app_data(
                        web::Data::new(Arc::clone(&regexword_component.store))
                    )
                    .app_data(
                        web::Data::new(Arc::clone(&regexword_component.journal))
                    )
                    .app_data(
                        web::Data::new(Arc::clone(&regexword_component.service))
                    )
                    .app_data(
                        web::Data::new(api_key_component.service.clone())
                    )
            )
    })
        .workers(2)
        .bind((api_address.clone(), api_port.clone()))?
        .run()
        .await
}