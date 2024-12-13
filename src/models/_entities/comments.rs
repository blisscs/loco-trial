//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    pub article_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::articles::Entity",
        from = "Column::ArticleId",
        to = "super::articles::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Articles,
}

impl Related<super::articles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Articles.def()
    }
}
