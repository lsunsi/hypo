use hypo::{div, element};

macro_rules! assert_render {
    ($hypo:expr, $ex:expr) => {{
        assert_eq!(hypo::render($hypo).0, $ex);
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
fn child_trailing() {
    assert_render!(div!(div!(),), "<div><div></div></div>");
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
fn attr_trailing() {
    assert_render!(div!(id = "idê",), "<div id=\"idê\"></div>");
}

#[test]
fn attr_kebab() {
    #[cfg(feature = "kebab")]
    let expected = "<div type=\"tipo\" hx-get=\"/\"></div>";
    #[cfg(not(feature = "kebab"))]
    let expected = "<div r#type=\"tipo\" hx_get=\"/\"></div>";
    assert_render!(div!(r#type = "tipo", hx_get = "/"), expected);
}

#[test]
fn partial_nested_attr() {
    let partial = |text: String| div!(div!(text));

    assert_render!(
        partial(String::from("oiblz")),
        "<div><div>oiblz</div></div>"
    );
}

#[test]
fn element_base() {
    assert_render!(
        element!("tag", id = "idê", div!()),
        "<tag id=\"idê\"><div></div></tag>"
    );
}

#[test]
fn element_void() {
    assert_render!(element!("tag" => void, id = "idê"), "<tag id=\"idê\">");
}
