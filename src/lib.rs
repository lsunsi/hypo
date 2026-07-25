mod attribute;
mod element;
mod escape;
mod r#macro;
mod render;

pub use attribute::Attributes;
pub use element::Element;
pub use render::{Fn, Raw, Render};

pub const DOCTYPE: Raw<&str> = Raw("<!DOCTYPE html>");
