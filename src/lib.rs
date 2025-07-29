#![feature(associated_type_defaults)]

#[cfg(feature = "main")]
#[allow(warnings)]
#[path = "../schema/generated/main_capnp.rs"]
pub(crate) mod main_capnp;

#[cfg(feature = "game")]
#[allow(warnings)]
#[path = "../schema/generated/game_capnp.rs"]
pub(crate) mod game_capnp;

#[cfg(feature = "srvc")]
#[allow(warnings)]
#[path = "../schema/generated/srvc_capnp.rs"]
pub(crate) mod srvc_capnp;

#[allow(warnings)]
#[path = "../schema/generated/shared_capnp.rs"]
pub(crate) mod shared_capnp;

pub mod config;
pub mod data;
pub mod encoding;
pub mod logging;
pub mod schema;
mod session_id;
pub mod token_issuer;

pub use session_id::SessionId;
