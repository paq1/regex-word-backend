use crate::api::regexword::regexword_dbo::RegexWordDboState;
use crate::core::regexword::data::states::RegexWordStates;
use crate::core::regexword::repositories::CustomRegexWordRepository;
use async_trait::async_trait;
use chrono::NaiveDate;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::core::repositories::entities::ReadOnlyEntityRepo;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::MongoEntityRepository;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};
use futures::StreamExt;
use log::{error, info, warn};
use mongodb::bson::{doc, Bson};

pub type MongoRegexWordRepository = MongoEntityRepository<RegexWordDboState>;

#[async_trait]
impl CustomRegexWordRepository for MongoRegexWordRepository {
    async fn fetch_one_word_random(&self, date: &NaiveDate) -> ResultErr<Option<Entity<RegexWordStates, String>>> {
        info!("select random world");
        let filter = doc! {
            "data.date_last_selected": {
                "$ne": date.to_string()
            }
        };

        let pipeline = vec![
            doc! { "$match": filter },
            doc! { "$sample": { "size": 1 } }
        ];

        let mut cursor = self.database
            .collection::<RegexWordDboState>("regexword_store")
            .aggregate(pipeline)
            .await
            .map_err(|err| Error::Simple(err.to_string()))?;

        match cursor.next().await {
            Some(Ok(doc)) => {

                // let dbo: RegexWordDboState = doc.into();
                info!("mot selectionne: {doc:?}");
                if let Some(Bson::String(entity_id)) = doc.get("id") {
                    self.fetch_one(entity_id).await
                } else {
                    error!("champs 'id' manquant");
                    Err(Error::Simple("pas de champs 'id' dans le document mongo".to_string()))
                }
            }
            Some(Err(err)) => Err(Error::Simple(err.to_string())),
            None => {
                warn!("Pas de document trouvé. Ce cas ne devrait pas arriver");
                Ok(None)
            },
        }

    }
}
