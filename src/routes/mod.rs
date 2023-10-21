mod connect4;
mod auth;
pub mod utils;

use self::utils::RouterInfo;

pub fn get_routes() -> RouterInfo {
    let c4 = RouterInfo::nest("/connect4", vec!(
        connect4::get_routes(),
    ));

    RouterInfo::nest("/api", vec!(c4,))
}
