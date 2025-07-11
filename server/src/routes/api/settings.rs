mod factors;

use super::Route;

pub fn routes() -> Vec<Route> {
    [factors::routes()].concat()
}
