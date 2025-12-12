use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(default_value = "uuid::Uuid::new_v4")]
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub preferred_username: String,
    pub email: String,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}
