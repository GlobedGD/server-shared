#![feature(associated_type_defaults, downcast_unchecked)]

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
pub mod hmac_signer;
pub mod logging;
mod multi_color;
pub mod schema;
mod session_id;
pub mod token_issuer;
mod typemap;
mod user_settings;

pub use multi_color::*;
pub use session_id::SessionId;
pub use typemap::TypeMap;
pub use user_settings::UserSettings;
