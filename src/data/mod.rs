mod capnp;
pub use capnp::*;

#[cfg(feature = "srvc")]
mod srvc;
use serde::Deserialize;
#[cfg(feature = "srvc")]
pub use srvc::*;

use crate::UsernameString;

// Note: if changing this struct, ensure it matches argon /v2/token/validate response or make a new struct for argon client
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ClientAccountData {
    pub account_id: i32,
    pub user_id: i32,
    pub username: UsernameString,
}
