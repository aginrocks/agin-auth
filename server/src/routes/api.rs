mod health;
mod login;

use super::Route;

pub fn routes() -> Vec<Route> {
    [health::routes(), login::routes()].concat()
}
