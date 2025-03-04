#![doc = include_str!("../README.md")]
#![cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    warn(missing_docs)
)]

#[allow(unused_macros)]
macro_rules! dispatch_inline_item {
    ( native: $native:item web: $web:item ) => {
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        $native

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        $web
    };
}
#[allow(unused_imports)]
use dispatch_inline_item;

#[allow(unused_macros)]
macro_rules! dispatch_inline_stmt {
    ( native: $native:stmt ; web: $web:stmt ; ) => {
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        $native;

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        $web;
    };
}
#[allow(unused_imports)]
use dispatch_inline_stmt;

macro_rules! dispatch_to_submodules {
    () => {
        crate::dispatch_inline_item!(
            native: mod native;
            web: mod web;
        );
        crate::dispatch_inline_item!(
            native: pub use native::*;
            web: pub use web::*;
        );
    };
}
use dispatch_to_submodules;

#[cfg(feature = "time")]
mod sleep;
#[cfg(feature = "time")]
pub use sleep::*;

#[cfg(feature = "spawn")]
mod spawn;
#[cfg(feature = "spawn")]
pub use spawn::*;

mod traits;
pub use traits::*;

#[cfg(feature = "websocket")]
mod websocket;
#[cfg(feature = "websocket")]
pub use websocket::*;
