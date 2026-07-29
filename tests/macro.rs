use hypo::{Render, div};

#[macro_export]
macro_rules! assert_render {
    ($hypo:expr, $ex:expr) => {{
        let mut s = String::new();
        $hypo.render(&mut s);
        assert_eq!(s, $ex);
    }};
}

#[test]
fn base() {
    assert_render!(div!(), "<div></div>");
}

#[test]
fn child() {
    assert_render!(div!(div!()), "<div><div></div></div>");
}

#[test]
fn children() {
    assert_render!(
        div!(div!(div!()), div!()),
        "<div><div><div></div></div><div></div></div>"
    );
}

#[test]
fn attr() {
    assert_render!(div!(id = "idê"), "<div id=\"idê\"></div>");
}

#[test]
fn attrs() {
    assert_render!(
        div!(id = "idê", disabled = true),
        "<div id=\"idê\" disabled></div>"
    );
}

#[test]
fn attr_kebab() {
    #[cfg(feature = "kebab")]
    let expected = "<div hx-get=\"/\"></div>";
    #[cfg(not(feature = "kebab"))]
    let expected = "<div hx_get=\"/\"></div>";
    assert_render!(div!(hx_get = "/"), expected);
}
