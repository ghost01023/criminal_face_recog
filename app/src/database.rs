use crate::entities::{criminal, criminal_photo};
use chrono::Utc;
use sea_orm::*;

#[derive(Debug, Clone)]
pub struct CriminalDB {
    pub connection: DatabaseConnection,
}

impl CriminalDB {
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let connection = Database::connect(db_url).await?;
        Ok(Self { connection })
    }

    pub async fn add_criminal(
        &self,
        name: String,
        fathers_name: Option<String>,
        arrested_location: Option<String>,
        no_of_crimes: u32,
    ) -> Result<u32, DbErr> {
        let new_criminal = criminal::ActiveModel {
            name: Set(name),
            fathers_name: Set(fathers_name),
            arrested_location: Set(arrested_location),
            no_of_crimes: Set(no_of_crimes),
            date_of_arrest: Set(Utc::now()),
            ..Default::default()
        };

        let result = criminal::Entity::insert(new_criminal)
            .exec(&self.connection)
            .await?;
        Ok(result.last_insert_id)
    }

    pub async fn get_criminal(&self, id: u32) -> Result<Option<criminal::Model>, DbErr> {
        criminal::Entity::find()
            .filter(criminal::Column::CriminalId.eq(id))
            .one(&self.connection)
            .await
    }

    /// Get all photos for a criminal
    pub async fn get_criminal_photos(
        &self,
        criminal_id: u32,
    ) -> Result<Vec<criminal_photo::Model>, DbErr> {
        criminal_photo::Entity::find()
            .filter(criminal_photo::Column::CriminalId.eq(criminal_id))
            .all(&self.connection)
            .await
    }

    /// Optional: Get criminal along with photos using relation
    pub async fn get_criminal_with_photos(
        &self,
        id: u32,
    ) -> Result<Option<(criminal::Model, Vec<criminal_photo::Model>)>, DbErr> {
        if let Some(criminal) = self.get_criminal(id).await? {
            let photos = self.get_criminal_photos(id).await?;
            Ok(Some((criminal, photos)))
        } else {
            Ok(None)
        }
    }

    pub async fn add_criminal_photo(
        &self,
        criminal_id: u32,
        photo_bytes: Vec<u8>,
    ) -> Result<u32, DbErr> {
        let new_photo = criminal_photo::ActiveModel {
            criminal_id: Set(criminal_id),
            photo: Set(photo_bytes),
            ..Default::default()
        };

        let result = criminal_photo::Entity::insert(new_photo)
            .exec(&self.connection)
            .await?;
        Ok(result.last_insert_id)
    }
}
