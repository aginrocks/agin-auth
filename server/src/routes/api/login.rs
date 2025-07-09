mod options;

use super::Route;

pub fn routes() -> Vec<Route> {
    [options::routes()].concat()
}
