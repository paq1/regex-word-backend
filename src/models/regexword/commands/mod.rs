use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum RegexWordCommands {
    Create(CreateRegexWordCommand),
    SelectOne(SelectOneRegexWordCommand),
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateRegexWordCommand {
    pub word: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct SelectOneRegexWordCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_regex: Option<bool>
}
