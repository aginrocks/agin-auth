use macros::register_factors;

pub mod password;

register_factors! {
    "password" => password::PasswordFactor,
}
