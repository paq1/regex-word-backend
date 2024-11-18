use crate::api::regexword::regexword_dbo::{RegexWordDboEvent, RegexWordDboState};
use framework_cqrs_lib::cqrs::infra::daos::mongo_entity_dao::{EntityMongoDAO, EventMongoDAO};

pub type RegexWordMongoDAO = EntityMongoDAO<RegexWordDboState>;
pub type RegexWordEventMongoDAO = EventMongoDAO<RegexWordDboEvent>;