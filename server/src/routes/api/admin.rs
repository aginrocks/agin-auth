pub mod applications;

use super::Route;

pub fn routes() -> Vec<Route> {
    [applications::routes()].concat()
}

// TODO: Add proper auth
