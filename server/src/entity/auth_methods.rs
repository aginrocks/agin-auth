use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_auth_methods")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[sea_orm(primary_key)]
    pub method_type: Method,

    /// A factor is enabled only after successful enrollment and confirmation (for example, TOTP code verified)
    #[sea_orm(default_value = "true")]
    pub is_enabled: bool,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub enrolled_at: DateTimeUtc,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub modified_at: DateTimeUtc,

    pub last_used_at: Option<DateTimeUtc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "user_role",
    rename_all = "lowercase"
)]
pub enum Method {
    #[sea_orm(string_value = "password")]
    Password,
    #[sea_orm(string_value = "totp")]
    Totp,
    #[sea_orm(string_value = "recovery_codes")]
    RecoveryCodes,
    #[sea_orm(string_value = "webauthn")]
    WebAuthn,
    #[sea_orm(string_value = "pgp")]
    Pgp,
    #[sea_orm(string_value = "mobile_authenticator")]
    MobileAuthenticator,
}

impl ActiveModelBehavior for ActiveModel {}
