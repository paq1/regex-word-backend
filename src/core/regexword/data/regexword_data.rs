use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct RegexWordData {
    pub word: String,
    pub regex_parts: Vec<String>,
    pub order: Vec<u32>,
    pub nb_selected: u32,
    pub date_last_selected: Option<NaiveDate>
}
