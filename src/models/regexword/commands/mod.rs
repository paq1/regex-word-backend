use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum RegexWordCommands {
    Create(CreateRegexWordCommand),
    Activate(ActivateRegexWordCommand),
    Disable(DisableRegexWordCommand),
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateRegexWordCommand {
    pub word: String,
    pub regex_parts: Vec<String>,
    pub niveau_difficulte: String, // TODO : mettre en place un enum
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ActivateRegexWordCommand {}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct DisableRegexWordCommand {}
