use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "criminals")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub criminal_id: u32,
    pub name: String,
    pub fathers_name: Option<String>,
    pub date_of_arrest: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    pub no_of_crimes: u32,
    pub arrested_location: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::criminal_photo::Entity")]
    Photos,
}

impl Related<super::criminal_photo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Photos.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // If last_seen is not set, default it to 16 hours before date_of_arrest
        if !self.last_seen.is_set() || self.last_seen.as_ref().is_none() {
            let arrest_date = match self.date_of_arrest {
                Set(ref date) => *date, // Copy the DateTime<Utc>
                _ => Utc::now(),        // fallback
            };
            self.last_seen = Set(Some(arrest_date - Duration::hours(16)));
        }
        Ok(self)
    }
}
