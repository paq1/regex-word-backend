use std::sync::Arc;

use crate::core::regexword::data::events::RegexWordEvents;
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::commands::{RegexWordCommands, CreateRegexWordCommand, DisableRegexWordCommand, ActivateRegexWordCommand};
use crate::models::regexword::views::RegexWordViewEvent;
use actix_web::{post, put, web, Responder};
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use framework_cqrs_lib::cqrs::infra::helpers::http_response::{CanToHttpResponse, HttpKindResponse};
use framework_cqrs_lib::cqrs::infra::mappers::event_api_view::from_entity_event_to_view;
use uuid::Uuid;
use crate::core::regexword::command_handler::activate_handler::RegexWordActivateHandler;
use crate::core::regexword::command_handler::create_handler::RegexWordCreateHandler;
use crate::core::regexword::command_handler::disable_handler::RegexWordDisableHandler;

#[utoipa::path(
    tag = "regexword",
    path = "/regexword/commands/create",
    request_body = CreateRegexWordCommand,
    responses(
    (status = 201, description = "mettre la description ici", body = String),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/commands/create")]
pub async fn insert_one_regexword(
    body: web::Json<CreateRegexWordCommand>,
    engine: web::Data<Arc<Engine<RegexWordStates, RegexWordCommands, RegexWordEvents>>>,
) -> impl Responder {
    let ctx = Context::empty();

    let command = RegexWordCommands::Create(body.into_inner());

    let entity_id = Uuid::new_v4().to_string();

    let event = engine
        .compute(command, entity_id, RegexWordCreateHandler::handler_name().to_string(), &ctx).await;

    event.map(|(event, _)| {
        from_entity_event_to_view::<RegexWordEvents, RegexWordViewEvent>(
            event,
            "regexword".to_string(),
            "urn:api:regexword:regexword".to_string(),
            &ctx,
        )
    })
        .to_http_response_with_error_mapping(HttpKindResponse::Created)
}

#[utoipa::path(
    tag = "regexword",
    path = "/regexword/{entity_id}/commands/activate",
    request_body = ActivateRegexWordCommand,
    responses(
    (status = 200, description = "fait ca", body = String),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/{entity_id}/commands/activate")]
pub async fn activate_one_regexword(
    path: web::Path<String>,
    body: web::Json<ActivateRegexWordCommand>,
    engine: web::Data<Arc<Engine<RegexWordStates, RegexWordCommands, RegexWordEvents>>>,
) -> impl Responder {
    let ctx = Context::empty();
    let id = path.into_inner();
    let command = RegexWordCommands::Activate(body.into_inner());

    let event = engine
        .compute(command, id, RegexWordActivateHandler::handler_name().to_string(), &ctx).await;

    event.map(|(event, _)| {
        from_entity_event_to_view::<RegexWordEvents, RegexWordViewEvent>(
            event,
            "regexword".to_string(),
            "urn:api:regexword:regexword".to_string(),
            &ctx,
        )
    })
        .to_http_response_with_error_mapping(HttpKindResponse::Ok)
}

#[utoipa::path(
    tag = "regexword",
    path = "/regexword/{entity_id}/commands/disable",
    request_body = DisableRegexWordCommand,
    responses(
    (status = 200, description = "???", body = String),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/{entity_id}/commands/disable")]
pub async fn disable_one_regexword(
    path: web::Path<String>,
    body: web::Json<DisableRegexWordCommand>,
    engine: web::Data<Arc<Engine<RegexWordStates, RegexWordCommands, RegexWordEvents>>>,
) -> impl Responder {
    let ctx = Context::empty();
    let id = path.into_inner();
    let command = RegexWordCommands::Disable(body.into_inner());

    let event = engine
        .compute(command, id, RegexWordDisableHandler::handler_name().to_string(), &ctx).await;


    event.map(|(event, _)| {
        from_entity_event_to_view::<RegexWordEvents, RegexWordViewEvent>(
            event,
            "regexword".to_string(),
            "urn:api:regexword:regexword".to_string(),
            &ctx,
        )
    })
        .to_http_response_with_error_mapping(HttpKindResponse::Ok)
}
