use qunet::server::builder::BufferPoolOpts;
/// Various helper functions for server configuration
///
use serde::de::DeserializeOwned;
use std::{error::Error, net::SocketAddr, path::PathBuf};
use tracing::error;

const S_4KIB: usize = 2usize.pow(12);
const S_8KIB: usize = 2usize.pow(13);
const S_16KIB: usize = 2usize.pow(14);
const S_32KIB: usize = 2usize.pow(15);
const S_64KIB: usize = 2usize.pow(16);
const S_128KIB: usize = 2usize.pow(17);
const S_256KIB: usize = 2usize.pow(18);
const S_512KIB: usize = 2usize.pow(19);
const S_1MIB: usize = 2usize.pow(20);
const S_2MIB: usize = 2usize.pow(21);
const S_4MIB: usize = 2usize.pow(22);
const S_8MIB: usize = 2usize.pow(23);
const S_16MIB: usize = 2usize.pow(24);
const S_32MIB: usize = 2usize.pow(25);
const S_64MIB: usize = 2usize.pow(26);
const S_128MIB: usize = 2usize.pow(27);
const S_256MIB: usize = 2usize.pow(28);
#[allow(unused)]
const S_512MIB: usize = 2usize.pow(29);
const S_1GIB: usize = 2usize.pow(30);

pub fn make_memory_limits(mut usage: u32) -> (usize, usize, Option<usize>, Option<usize>) {
    usage = usage.clamp(1, 11);

    match usage {
        1 => (S_16KIB, S_64KIB, None, None),
        2 => (S_64KIB, S_256KIB, None, None),
        3 => (S_256KIB, S_1MIB, None, None),
        4 => (S_512KIB, S_4MIB, None, None),
        5 => (S_1MIB, S_8MIB, None, None),
        6 => (S_2MIB, S_16MIB, None, None),
        7 => (S_4MIB, S_32MIB, None, Some(S_512KIB)),
        8 => (S_8MIB, S_64MIB, None, Some(S_1MIB)),
        9 => (S_16MIB, S_128MIB, Some(S_512KIB), Some(S_2MIB)),
        10 => (S_16MIB, S_256MIB, Some(S_1MIB), Some(S_4MIB)),
        11 => (S_32MIB, S_1GIB, Some(S_2MIB), Some(S_8MIB)),
        _ => unreachable!(),
    }
}

/// Primarily for game server.
#[allow(unused)]
pub fn make_udp_memory_limits(mut usage: u32) -> BufferPoolOpts {
    usage = usage.clamp(1, 11);

    let (min_bufs, max_bufs) = match usage {
        1 => (4, 32),
        2 => (8, 64),
        3 => (16, 128),
        4 => (16, 256),
        5 => (16, 512),
        6 => (32, 1024),
        7 => (32, S_4KIB),
        8 => (64, S_8KIB),
        9 => (128, S_16KIB),
        10 => (256, S_32KIB),
        11 => (512, S_128KIB),
        _ => unreachable!(),
    };

    BufferPoolOpts::new(1500, min_bufs, max_bufs)
}

pub fn log_buffer_size_for_memlimit(mut usage: u32) -> usize {
    usage = usage.clamp(1, 11);

    match usage {
        1 => 16,
        2 => 64,
        3 => 256,
        4 => 1024,
        5 => 2048,
        6 => S_4KIB,
        7 => S_8KIB,
        8 => S_16KIB,
        9 => S_32KIB,
        10 => S_64KIB,
        11 => S_128KIB,
        _ => unreachable!(),
    }
}

pub fn parse_addr(addr: &str, name: &str) -> SocketAddr {
    match addr.parse() {
        Ok(x) => x,
        Err(e) => {
            error!("failed to parse option '{name}': {e}");
            error!(
                "note: it must be a valid IPv4/IPv6 socket address, for example \"0.0.0.0:4340\" or \"[::]:4340\""
            );

            std::process::exit(1);
        }
    }
}

pub trait CustomDeserialize: DeserializeOwned {
    fn deserialize(s: &str) -> Result<Self, Box<dyn Error>>;
}

/// Reads an environment variable with the given name, and if present, replaces the value of `val` with the parsed value.
pub fn env_replace<T: CustomDeserialize>(varname: &str, val: &mut T) {
    if let Ok(var) = std::env::var(varname) {
        match <T as CustomDeserialize>::deserialize(&var) {
            Ok(v) => *val = v,
            Err(e) => {
                eprintln!("Failed to parse environment variable {varname}: {e}");
                std::process::exit(1);
            }
        };
    }
}

impl CustomDeserialize for String {
    fn deserialize(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.to_string())
    }
}

impl CustomDeserialize for PathBuf {
    fn deserialize(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(PathBuf::from(s))
    }
}

impl<T: CustomDeserialize> CustomDeserialize for Option<T> {
    fn deserialize(s: &str) -> Result<Self, Box<dyn Error>> {
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(<T as CustomDeserialize>::deserialize(s)?))
        }
    }
}

impl CustomDeserialize for bool {
    fn deserialize(s: &str) -> Result<Self, Box<dyn Error>> {
        match s.to_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            _ => Err(format!("invalid boolean value: {s}").into()),
        }
    }
}

macro_rules! impl_custom_deserialize_int {
    ( $($t:ty),* ) => {
        $(
            impl CustomDeserialize for $t {
                fn deserialize(s: &str) -> Result<Self, Box<dyn Error>> {
                    match s.parse::<$t>() {
                        Ok(x) => Ok(x),
                        Err(e) => Err(format!("failed to parse integer: {e}").into()),
                    }
                }
            }
        )*
    };
}

impl_custom_deserialize_int!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, f32, f64);
