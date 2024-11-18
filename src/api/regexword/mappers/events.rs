use crate::api::regexword::regexword_dbo::{BandeauCreatedDbo, RegexWordDboEvent, BandeauDisabledDbo, BandeauProdUpDbo, BandeauUpdatedDbo};
use crate::core::regexword::data::events::{RegexWordActivated, RegexWordDisabled, RegexWordEvents, BandeauProdUp, BandeauUpdated};
use framework_cqrs_lib::cqrs::core::data::EntityEvent;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EventDBO;

impl From<RegexWordDboEvent> for RegexWordEvents {
    fn from(value: RegexWordDboEvent) -> Self {
        match value {
            RegexWordDboEvent::Created(event_dbo) =>
                RegexWordEvents::Activated(RegexWordActivated {
                    by: event_dbo.by,
                    at: event_dbo.at,
                    data: event_dbo.data.into(),
                }),
            RegexWordDboEvent::Updated(event_dbo) =>
                RegexWordEvents::Updated(
                    BandeauUpdated {
                        by: event_dbo.by,
                        at: event_dbo.at,
                        data: event_dbo.data.into(),
                    }
                ),
            RegexWordDboEvent::Disable(event_dbo) =>
                RegexWordEvents::Disabled(
                    RegexWordDisabled {
                        by: event_dbo.by,
                        at: event_dbo.at,
                        reason: event_dbo.reason,
                    }
                ),
            RegexWordDboEvent::Prod(event_dbo) =>
                RegexWordEvents::Prod(
                    BandeauProdUp {
                        by: event_dbo.by,
                        at: event_dbo.at,
                    }
                )
        }
    }
}

pub fn from_regexword_event_dbo_to_event(dbo: EventDBO<RegexWordDboEvent, String>) -> EntityEvent<RegexWordEvents, String> {
    EntityEvent {
        entity_id: dbo.entity_id,
        data: dbo.data.into(),
        event_id: dbo.event_id,
    }
}

pub fn from_regexword_event_to_dbo(dbo: EntityEvent<RegexWordEvents, String>) -> EventDBO<RegexWordDboEvent, String> {
    EventDBO {
        id_mongo: None,
        version: None,
        entity_id: dbo.entity_id,
        data: dbo.data.into(),
        event_id: dbo.event_id,
    }
}

impl From<RegexWordEvents> for RegexWordDboEvent {
    fn from(value: RegexWordEvents) -> Self {
        match value {
            RegexWordEvents::Activated(
                RegexWordActivated {
                    by,
                    at,
                    data
                }
            ) => RegexWordDboEvent::Created(BandeauCreatedDbo { by, at, data: data.into() }),
            RegexWordEvents::Updated(updated) =>
                RegexWordDboEvent::Updated(
                    BandeauUpdatedDbo {
                        by: updated.by,
                        at: updated.at,
                        data: updated.data.into(),
                    }
                ),
            RegexWordEvents::Disabled(disabled) =>
                RegexWordDboEvent::Disable(
                    BandeauDisabledDbo {
                        by: disabled.by,
                        at: disabled.at,
                        reason: disabled.reason,
                    }
                ),
            RegexWordEvents::Prod(prod) =>
                RegexWordDboEvent::Prod(
                    BandeauProdUpDbo {
                        by: prod.by,
                        at: prod.at,
                    }
                )
        }
    }
}

