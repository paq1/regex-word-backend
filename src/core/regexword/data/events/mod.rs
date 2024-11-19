use chrono::{DateTime, Utc};

use crate::core::regexword::data::regexword_data::RegexWordData;
use crate::models::regexword::views::{RegexWordCreatedView, RegexWordDataView, RegexWordIncrementedView, RegexWordViewEvent};
use framework_cqrs_lib::cqrs::models::jsonapi::CanBeView;

impl CanBeView<RegexWordViewEvent> for RegexWordEvents {
    fn to_view(&self) -> RegexWordViewEvent {
        match self.clone() {
            RegexWordEvents::Created(c) => RegexWordViewEvent::Created(RegexWordCreatedView {
                data: RegexWordDataView {
                    regex_parts: c.data.regex_parts,
                    word: c.data.word,
                    niveau_difficulte: c.data.niveau_difficulte,
                    nb_selected: c.data.nb_selected
                },
                by: c.by,
                at: c.at
            }),
            RegexWordEvents::Incremented(event) => RegexWordViewEvent::Activated(RegexWordIncrementedView {
                by: event.by,
                at: event.at,
                nb_selected: event.nb_selected,
            }),
        }
    }
}


#[derive(Clone)]
pub enum RegexWordEvents {
    Created(RegexWordCreated),
    Incremented(RegexWordIncremented),
}

#[derive(Clone)]
pub struct RegexWordCreated {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: RegexWordData,
}



#[derive(Clone)]
pub struct RegexWordIncremented {
    pub by: String,
    pub at: DateTime<Utc>,
    pub nb_selected: u32,
}
