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
pub struct SelectOneRegexWordCommand {}
