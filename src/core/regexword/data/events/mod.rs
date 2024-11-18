use chrono::{DateTime, Utc};

use crate::core::regexword::data::regexword_data::RegexWordData;
use crate::models::regexword::views::{RegexWordActivatedView, RegexWordDisabledView, RegexWordViewEvent, RegexWordCreatedView, RegexWordDataView};
use framework_cqrs_lib::cqrs::models::jsonapi::CanBeView;

impl CanBeView<RegexWordViewEvent> for RegexWordEvents {
    fn to_view(&self) -> RegexWordViewEvent {
        match self.clone() {
            RegexWordEvents::Created(c) => RegexWordViewEvent::Created(RegexWordCreatedView {
                data: RegexWordDataView {
                    regex_parts: c.data.regex_parts,
                    word: c.data.word,
                    niveau_difficulte: c.data.niveau_difficulte,
                },
                by: c.by,
                at: c.at
            }),
            RegexWordEvents::Activated(c) => RegexWordViewEvent::Activated(RegexWordActivatedView {
                by: c.by,
                at: c.at,
            }),
            RegexWordEvents::Disabled(c) => RegexWordViewEvent::Disabled(RegexWordDisabledView {
                by: c.by,
                at: c.at,
            }),
        }
    }
}


#[derive(Clone)]
pub enum RegexWordEvents {
    Created(RegexWordCreated),
    Activated(RegexWordActivated),
    Disabled(RegexWordDisabled),
}

#[derive(Clone)]
pub struct RegexWordCreated {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: RegexWordData,
}



#[derive(Clone)]
pub struct RegexWordActivated {
    pub by: String,
    pub at: DateTime<Utc>,
}



#[derive(Clone)]
pub struct RegexWordDisabled {
    pub by: String,
    pub at: DateTime<Utc>,
}
