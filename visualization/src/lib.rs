//! In-browser visualization for myelin using a canvas with WASM

#![feature(tool_lints)]
#![deny(rust_2018_idioms)]

pub mod bootstrapper;
pub mod input_handler;
mod presenter;
mod simulation;
pub mod view;
mod view_model;
