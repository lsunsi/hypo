#![doc = include_str!("../README.md")]

mod attribute;
mod escape;
mod r#macro;
mod render;

pub use attribute::Attributes;
pub use render::{Fn, Raw, Render};

/// the canonical html doctype
pub const DOCTYPE: Raw<&str> = Raw("<!DOCTYPE html>");

#[cfg(feature = "kebab")]
pub use const_str;
