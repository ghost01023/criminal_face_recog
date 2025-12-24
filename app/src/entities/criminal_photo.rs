use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "criminal_photos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub photo_id: u32,
    pub criminal_id: u32,
    // Vec<u8> auto maps to a binary/blob column
    pub photo: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::criminal::Entity",
        from = "Column::CriminalId",
        to = "super::criminal::Column::CriminalId",
        on_delete = "Cascade"
    )]
    Criminal,
}

impl Related<super::criminal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Criminal.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
