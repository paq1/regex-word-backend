use crate::api::regexword::regexword_dbo::RegexWordDboState;
use crate::core::regexword::repositories::CustomRegexWordRepository;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::MongoEntityRepository;

pub type MongoRegexWordRepository = MongoEntityRepository<RegexWordDboState>;

impl CustomRegexWordRepository for MongoRegexWordRepository {}
