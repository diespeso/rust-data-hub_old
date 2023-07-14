use sea_orm::entity::prelude::*;

use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "daily_forecast")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    //pub day: String,
    pub day: NaiveDateTime,
    pub max_temp: i32,
    pub min_temp: i32,
    pub register_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    
}

impl ActiveModelBehavior for ActiveModel {}