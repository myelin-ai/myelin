//! In-browser visualization for myelin using a canvas with WASM

#![feature(tool_lints)]
#![deny(rust_2018_idioms)]

pub mod bootstrapper;
pub mod input_handler;
pub(crate) mod presenter;
pub(crate) mod simulation;
pub(crate) mod view;
pub(crate) mod view_model;
