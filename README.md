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
# let mut s = String::new();
# page.render(&mut s);
# assert_eq!(s, r#"<html><head><meta charset="UTF-8"><title>hypotext</title></head><body class="container">leads to hypertext</body></html>"#);
```

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
# let mut s = String::new();
# section(None, Vec::new()).render(&mut s);
# assert_eq!(s, r#"<section>no rows</section>"#);
# let mut s = String::new();
# section(Some("oiblz"), Vec::new()).render(&mut s);
# assert_eq!(s, r#"<section><h1>¡oiblz!</h1>no rows</section>"#);
# let mut s = String::new();
# section(None, vec!["oi", "blz"]).render(&mut s);
# assert_eq!(s, r#"<section><ul><li>oi</li><li>blz</li></ul></section>"#);
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
