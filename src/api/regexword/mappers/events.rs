use crate::api::regexword::regexword_dbo::{RegexWordCreatedDbo, RegexWordDboEvent, RegexWordSelectedDbo};
use crate::core::regexword::data::events::{RegexWordSelected, RegexWordCreated, RegexWordEvents};
use framework_cqrs_lib::cqrs::core::data::EntityEvent;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EventDBO;

impl From<RegexWordDboEvent> for RegexWordEvents {
    fn from(value: RegexWordDboEvent) -> Self {
        match value {
            RegexWordDboEvent::Created(event_dbo) =>
                RegexWordEvents::Created(RegexWordCreated {
                    by: event_dbo.by,
                    at: event_dbo.at,
                    data: event_dbo.data.into(),
                }),
            RegexWordDboEvent::Selected(event_dbo) =>
                RegexWordEvents::Selected(
                    RegexWordSelected {
                        by: event_dbo.by,
                        at: event_dbo.at,
                        nb_selected: event_dbo.nb_selected,
                        date_last_selected: event_dbo.date_last_selected,
                        regex_parts: event_dbo.regex_parts,
                    }
                ),
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
            RegexWordEvents::Selected(
                RegexWordSelected {
                    by,
                    at,
                    nb_selected,
                    date_last_selected,
                    regex_parts
                }
            ) => RegexWordDboEvent::Selected(RegexWordSelectedDbo { by, at, nb_selected, date_last_selected, regex_parts }),
            RegexWordEvents::Created(created) =>
                RegexWordDboEvent::Created(
                    RegexWordCreatedDbo {
                        by: created.by,
                        at: created.at,
                        data: created.data.into(),
                    }
                ),
        }
    }
}

