#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate libc;

pub mod al;
pub mod alc;
pub mod efx;
pub mod ext;

#[cfg(feature = "presets")]
pub mod ambisonic_presets;
#[cfg(feature = "hrtf")]
pub mod hrtf;

#[cfg_attr(all(feature = "static", target_os = "linux"), link(name = "openal", kind = "static"))]
#[cfg_attr(all(not(feature = "static"), target_os = "linux"), link(name = "openal"))]
#[cfg_attr(all(feature = "static", target_os = "macos"), link(name = "OpenAL", kind = "static"))]
#[cfg_attr(all(not(feature = "static"), target_os = "macos"), link(name = "OpenAL", kind = "framework"))]
#[cfg_attr(all(feature = "static", target_os = "windows"), link(name = "OpenAL32", kind = "static"))]
#[cfg_attr(all(not(feature = "static"), target_os = "windows"), link(name = "OpenAL32"))]
extern {}

pub mod ffi {
    pub use super::al::ffi::*;
    pub use super::alc::ffi::*;
    pub use super::efx::ffi::*;
    pub use super::ext::ffi::*;
}

pub mod types {
    pub use super::al::types::*;
    pub use super::alc::types::*;
    pub use super::efx::types::*;
    pub use super::ext::types::*;
}

pub mod consts {
    pub use super::al::consts::*;
    pub use super::alc::consts::*;
    pub use super::efx::consts::*;
    pub use super::ext::consts::*;
}

pub mod all {
    pub use super::ffi::*;
    pub use super::types::*;
    pub use super::consts::*;
}