use chrono::{DateTime, NaiveDate, Utc};

use crate::core::regexword::data::regexword_data::RegexWordData;
use crate::models::regexword::views::{RegexWordCreatedView, RegexWordDataView, RegexWordSelectedView, RegexWordViewEvent};
use framework_cqrs_lib::cqrs::models::jsonapi::CanBeView;

impl CanBeView<RegexWordViewEvent> for RegexWordEvents {
    fn to_view(&self) -> RegexWordViewEvent {
        match self.clone() {
            RegexWordEvents::Created(c) => RegexWordViewEvent::Created(RegexWordCreatedView {
                data: RegexWordDataView {
                    regex_parts: c.data.regex_parts,
                    word: c.data.word,
                    order: c.data.order,
                    nb_selected: c.data.nb_selected,
                    date_last_selected: c.data.date_last_selected,
                },
                by: c.by,
                at: c.at
            }),
            RegexWordEvents::Selected(event) => RegexWordViewEvent::Selected(RegexWordSelectedView {
                by: event.by,
                at: event.at,
                nb_selected: event.nb_selected,
                date_last_selected: event.date_last_selected,
                regex_parts: event.regex_parts,
            }),
        }
    }
}


#[derive(Clone)]
pub enum RegexWordEvents {
    Created(RegexWordCreated),
    Selected(RegexWordSelected),
}

#[derive(Clone)]
pub struct RegexWordCreated {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: RegexWordData,
}



#[derive(Clone)]
pub struct RegexWordSelected {
    pub by: String,
    pub at: DateTime<Utc>,
    pub nb_selected: u32,
    pub date_last_selected: NaiveDate,
    pub regex_parts: Option<Vec<String>>,
}
