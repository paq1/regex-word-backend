use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct RegexWordData {
    pub word: String,
    pub regex_parts: Vec<String>,
    pub niveau_difficulte: String, // TODO : mettre en place un enum
    pub nb_selected: u32,
    pub date_last_selected: Option<NaiveDate>
}
