use std::sync::Arc;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};
use log::warn;
use reqwest::{Client, StatusCode};
use regex_word_backend::models::regexword::commands::CreateRegexWordCommand;

#[tokio::main]
async fn main() -> ResultErr<()>{
    dotenv::dotenv().ok();
    env_logger::init();

    let dictionnaire_service = dictionnaire::services::dictionnaire_service::DictionnaireService::new();

    let nb = dictionnaire_service.words.len();

    println!("nombre de mots : {nb}");

    let http_client: Arc<Client> = Arc::new(Client::new());

    for w in dictionnaire_service.words.iter() {
        let cmd = CreateRegexWordCommand {
            word: w.to_string()
        };

        let response = http_client
            .post("http://localhost:8080/regexword/commands/create")
            .json(&cmd)
            .send()
            .await
            .map_err(|e| Error::Simple(e.to_string()))?;

        let expected_status = StatusCode::from_u16(201).map_err(|e| Error::Simple(e.to_string()))?;

        if response.status() != expected_status {
            warn!("la création n'est pas OK");
        }
    }
    Ok(())
}
