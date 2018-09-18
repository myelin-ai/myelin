//! In-browser visualization for myelin using a canvas with WASM

#![feature(tool_lints)]
#![deny(rust_2018_idioms)]

pub mod bootstrapper;
pub(crate) mod controller;
pub mod input_handler;
pub(crate) mod presenter;
pub mod view;
pub(crate) mod view_model;
