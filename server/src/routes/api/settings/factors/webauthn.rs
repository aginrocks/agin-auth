mod finish;
mod start;

use super::Route;

pub fn routes() -> Vec<Route> {
    [start::routes(), finish::routes()].concat()
}
