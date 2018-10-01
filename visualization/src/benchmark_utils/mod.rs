#[cfg(target_arch = "wasm32")]
mod wasm32;

#[cfg(not(target_arch = "wasm32"))]
mod std;

#[cfg(target_arch = "wasm32")]
pub(crate) use self::wasm32::*;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use self::std::*;
