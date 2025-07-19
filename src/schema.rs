#[cfg(feature = "main")]
pub mod main {
    #[allow(unused)]
    pub use crate::main_capnp::*;
}

#[cfg(feature = "game")]
pub mod game {
    #[allow(unused)]
    pub use crate::game_capnp::*;
}

#[cfg(feature = "srvc")]
pub mod srvc {
    #[allow(unused)]
    pub use crate::srvc_capnp::*;
}

pub mod shared {
    #[allow(unused)]
    pub use crate::shared_capnp::*;
}
