use crate::core::regexword::data::states::RegexWordStates;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::repositories::entities::RepositoryEntity;


#[async_trait]
pub trait CustomRegexWordRepository: RepositoryEntity<RegexWordStates, String> {
    // TODO : ajouter les queries specifiques a l'ontology regexword
}
