#![doc = include_str!("../README.md")]

mod attrs;
mod escape;
mod r#macro;
mod render;

pub use attrs::Attrs;
pub use render::{Raw, Render};

/// the canonical html doctype
pub const DOCTYPE: Raw<&str> = Raw("<!DOCTYPE html>");

#[cfg(feature = "kebab")]
pub use const_str;
