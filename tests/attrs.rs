use hypo::{Attrs, render};

#[test]
fn single() {
    let s = Attrs(("id", "idê", ()));
    assert_eq!(render(s).0, " id=\"idê\">");
}

#[test]
fn many() {
    let s = Attrs(("value", 13, ("id", "idê", ())));
    assert_eq!(render(s).0, " id=\"idê\" value=\"13\">");
}

#[test]
fn consecutive() {
    let s = Attrs(("class", "yellow", ("class", "red", ())));
    assert_eq!(render(s).0, " class=\"red yellow\">");
}

#[test]
fn empty_single() {
    let s = Attrs(("value", (), ()));
    assert_eq!(render(s).0, ">");
}

#[test]
fn empty_before() {
    let s = Attrs(("class", "red", ("class", (), ())));
    assert_eq!(render(s).0, " class=\"red\">");
}

#[test]
fn empty_after() {
    let s = Attrs(("class", (), ("class", "red", ())));
    assert_eq!(render(s).0, " class=\"red\">");
}

#[test]
fn empty_between() {
    let s = Attrs(("class", "yellow", ("class", (), ("class", "red", ()))));
    assert_eq!(render(s).0, " class=\"red yellow\">");
}

#[test]
fn empty_between_skip() {
    let s = Attrs(("class", "yellow", ("id", (), ("class", "red", ()))));
    assert_eq!(render(s).0, " class=\"red yellow\">");
}

#[test]
fn bool_true() {
    let s = Attrs(("checked", true, ()));
    assert_eq!(render(s).0, " checked>");
}

#[test]
fn bool_false() {
    let s = Attrs(("checked", false, ()));
    assert_eq!(render(s).0, ">");
}

#[test]
fn bool_true_true() {
    let s = Attrs(("checked", true, ("checked", true, ())));
    assert_eq!(render(s).0, " checked>");
}

#[test]
fn bool_true_false() {
    let s = Attrs(("checked", false, ("checked", true, ())));
    assert_eq!(render(s).0, " checked>");
}

#[test]
fn bool_false_true() {
    let s = Attrs(("checked", true, ("checked", false, ())));
    assert_eq!(render(s).0, " checked>");
}

#[test]
fn bool_false_false() {
    let s = Attrs(("checked", false, ("checked", false, ())));
    assert_eq!(render(s).0, ">");
}

#[test]
fn bool_true_false_true() {
    let s = Attrs(("checked", true, ("checked", false, ("checked", true, ()))));
    assert_eq!(render(s).0, " checked>");
}
