mod challenge;

use super::Route;

pub fn routes() -> Vec<Route> {
    [challenge::routes()].concat()
}
