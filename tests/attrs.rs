use hypo::{Attrs, Render};

#[test]
fn single() {
    let mut s = String::new();
    Attrs(("id", "idê", ())).render(&mut s);
    assert_eq!(s, " id=\"idê\">");
}

#[test]
fn many() {
    let mut s = String::new();
    Attrs(("value", 13, ("id", "idê", ()))).render(&mut s);
    assert_eq!(s, " id=\"idê\" value=\"13\">");
}

#[test]
fn consecutive() {
    let mut s = String::new();
    Attrs(("class", "yellow", ("class", "red", ()))).render(&mut s);
    assert_eq!(s, " class=\"red yellow\">");
}

#[test]
fn empty_single() {
    let mut s = String::new();
    Attrs(("value", (), ())).render(&mut s);
    assert_eq!(s, ">");
}

#[test]
fn empty_before() {
    let mut s = String::new();
    Attrs(("class", "red", ("class", (), ()))).render(&mut s);
    assert_eq!(s, " class=\"red\">");
}

#[test]
fn empty_after() {
    let mut s = String::new();
    Attrs(("class", (), ("class", "red", ()))).render(&mut s);
    assert_eq!(s, " class=\"red\">");
}

#[test]
fn empty_between() {
    let mut s = String::new();
    Attrs(("class", "yellow", ("class", (), ("class", "red", ())))).render(&mut s);
    assert_eq!(s, " class=\"red yellow\">");
}

#[test]
fn empty_between_skip() {
    let mut s = String::new();
    Attrs(("class", "yellow", ("id", (), ("class", "red", ())))).render(&mut s);
    assert_eq!(s, " class=\"red yellow\">");
}

#[test]
fn bool_true() {
    let mut s = String::new();
    Attrs(("checked", true, ())).render(&mut s);
    assert_eq!(s, " checked>");
}

#[test]
fn bool_false() {
    let mut s = String::new();
    Attrs(("checked", false, ())).render(&mut s);
    assert_eq!(s, ">");
}

#[test]
fn bool_true_true() {
    let mut s = String::new();
    Attrs(("checked", true, ("checked", true, ()))).render(&mut s);
    assert_eq!(s, " checked>");
}

#[test]
fn bool_true_false() {
    let mut s = String::new();
    Attrs(("checked", false, ("checked", true, ()))).render(&mut s);
    assert_eq!(s, " checked>");
}

#[test]
fn bool_false_true() {
    let mut s = String::new();
    Attrs(("checked", true, ("checked", false, ()))).render(&mut s);
    assert_eq!(s, " checked>");
}

#[test]
fn bool_false_false() {
    let mut s = String::new();
    Attrs(("checked", false, ("checked", false, ()))).render(&mut s);
    assert_eq!(s, ">");
}

#[test]
fn bool_true_false_true() {
    let mut s = String::new();
    Attrs(("checked", true, ("checked", false, ("checked", true, ())))).render(&mut s);
    assert_eq!(s, " checked>");
}
