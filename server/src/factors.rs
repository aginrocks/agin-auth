use macros::register_factors;

pub mod password;
pub mod pgp;

register_factors! {
    "password" => password::PasswordFactor,
    "pgp" => pgp::PgpFactor,
}
