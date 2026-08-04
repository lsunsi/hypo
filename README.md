# hypo
[![🛠️🔒](https://github.com/lsunsi/hypo/actions/workflows/rust.yml/badge.svg)](https://github.com/lsunsi/hypo/actions/workflows/rust.yml)

A tiny crate that renders html through macros.

***Hypo**text is the source leading to the hypertext*.

## Design
- *Minimal interface*
  - One macro per element, which take attributes and children
  - Composition favored over a complex DSL
- *Formatting support*
  - Macros are used in functional style
  - Syntax is chosen carefully to be accepted by rustfmt
- *Zero intermediary allocations*
  - Only allocates on full template render
  - Render without std if buffer is available
- *Zero procedural macros*
  - Only declarative macros
  - Even those are tiny and simple
- *Zero dependencies*
  - Can be used with zero dependencies
  - Couple dependencies might be enabled via features

## Usage
```rust
use hypo::*;
let page = html!(
    head!(
        meta!(charset = "UTF-8"),
        title!("hypotext")
    ),
    body!(
        class = "container",
        "leads to hypertext"
    )
);
# assert_eq!(render(page).0, r#"<html><head><meta charset="UTF-8"><title>hypotext</title></head><body class="container">leads to hypertext</body></html>"#);
```

## Render trait
The `Render` trait is the bedrock of this library. Every value interpolated into the markup needs to implement it.

The interpolation happens basically in two places:
- **Children** of elements `div!(children)`
- **Value** of attributes `div!(key = value)`

In order to understand this crate design, it's important to keep in mind the types that implement this trait. Here's a quick list of the most important ones and their behavior.

- `char`, `&str` and `String` (implies escaping)
- `Raw` (defined in this library, string that skips escaping)
- all integer types (implies allocation without `perf` feature)
- `Vec`, `array` and `Map` (concatenation of items)
- tuples (size up to 16, concatenation too)
- `Option` (renders nothing on None)
- `Result` (just renders both sides)

## Control flow
This crate does not provide any syntax for control flow primitives.

It prioritize native structs that implement `Render` so the user can use it's combinatory methods and primitives to achieve some interesting control flow results.

```rust
# use hypo::*;
let section = |header: Option<&'static str>, rows: Vec<&'static str>| {
    section!(
        // if-let adjacent
        header.map(|h| h1!(('¡', h, '!'))),
        
        // if adjacent
        (!rows.is_empty()).then_some(
            // for adjacent
            ul!(rows.into_iter().map(|row| li!(row)))
        )
        // else adjacent
        .ok_or("no rows")
    )
};
# assert_eq!(render(section(None, Vec::new())).0, r#"<section>no rows</section>"#);
# assert_eq!(render(section(Some("oiblz"), Vec::new())).0, r#"<section><h1>¡oiblz!</h1>no rows</section>"#);
# assert_eq!(render(section(None, vec!["oi", "blz"])).0, r#"<section><ul><li>oi</li><li>blz</li></ul></section>"#);
```

## Note on attributes
This crate design takes special care to support some common cases on real world situations. At this point in the documentation, you already know everything you need to figure this out yourself, but I'll mention three examples that might not be that obvious at first glance.

```rust
# use hypo::*;
# let mut s = String::new();
// Some renders attribute normally
input!(value = Some("oiblz")).render(&mut s);
assert_eq!(s, r#"<input value="oiblz">"#);
# s.clear();
// None means nothing gets rendered and even attribute key is taken out
input!(value = None::<&str>).render(&mut s);
assert_eq!(s, r#"<input>"#);
```
```rust
# use hypo::*;
# let mut s = String::new();
let path = "/about";
a!(href = ("https://oiblz", path)).render(&mut s);
// tuple concatenates, so it works for interpolation without allocation
assert_eq!(s, r#"<a href="https://oiblz/about"></a>"#);
```
```rust
# use hypo::*;
# let mut s = String::new();
// boolean values are for boolean attributes
select!(checked = true).render(&mut s);
assert_eq!(s, r#"<select checked></select>"#);
# s.clear();
// false omits the attribute, true places it without value
select!(checked = false).render(&mut s);
assert_eq!(s, r#"<select></select>"#);
```
```rust
# use hypo::*;
# let mut s = String::new();
// consecutive attribute keys means join with space
button!(class = "bg-red", class = true.then_some("warning")).render(&mut s);
// so in this case you can have a class always present, while another is optional
assert_eq!(s, r#"<button class="bg-red warning"></button>"#);
```

## Feature flags
No features are enabled by default, which means this crate does not carry any dependencies. But hear me out...

- **kebab**: Attributes will have the underline (_) replaced by a dash (-) in compile time. This is useful for cleanly adding an `hx-get` for example, for no runtime cost.

- **perf**: The Render implementation for numbers won't use Display, instead leveraging a performance oriented library. This might give you a rendering performance boost if you have lots of numbers.

## Versus

- [vy](https://github.com/JonahLund/vy), the main inspiration of this library.
  - Hypotext has syntax that allows for conditional attributes and classes, without giving up formatting.
  - Hypotext has no procedural macros, even with all features enabled.
  - I'd pick hypotext over it because it's simpler and more powerful.

- [maud](https://github.com/lambda-fairy/maud), de facto compiled html macro dsl.
  - Hypotext has way less dsl, which could work as a pro or a con.
  - Hypotext has no procedural macros, even with all features enabled.
  - Hypotext does not allocate intermediary containers.
  - I'd pick maud if you prefer it's dsl.

- [askama](https://github.com/askama-rs/askama), actual html templates jinja style.
  - Hypotext has a simpler syntax for composition (just functions).
  - Hypotext will avoid the html change of context, which could work as a pro or con.
  - I'd pick askama if you want to have raw html files.

###### Thanks for reading all of this.
