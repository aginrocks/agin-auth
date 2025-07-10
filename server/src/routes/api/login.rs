mod options;
mod password;

use super::Route;

pub fn routes() -> Vec<Route> {
    [options::routes(), password::routes()].concat()
}
