mod enable;

use super::Route;

pub fn routes() -> Vec<Route> {
    [enable::routes()].concat()
}
