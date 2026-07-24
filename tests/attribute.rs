use hypo::Attributes;

#[test]
fn single() {
    let mut s = String::new();
    Attributes(("id", "idê", ())).render(&mut s);
    assert_eq!(s, " id=\"idê\">");
}

#[test]
fn many() {
    let mut s = String::new();
    Attributes(("value", 13, ("id", "idê", ()))).render(&mut s);
    assert_eq!(s, " id=\"idê\" value=\"13\">");
}

#[test]
fn consecutive() {
    let mut s = String::new();
    Attributes(("class", "yellow", ("class", "red", ()))).render(&mut s);
    assert_eq!(s, " class=\"red yellow\">");
}

#[test]
fn empty_single() {
    let mut s = String::new();
    Attributes(("value", (), ())).render(&mut s);
    assert_eq!(s, ">");
}

#[test]
fn empty_before() {
    let mut s = String::new();
    Attributes(("class", "red", ("class", (), ()))).render(&mut s);
    assert_eq!(s, " class=\"red\">");
}

#[test]
fn empty_after() {
    let mut s = String::new();
    Attributes(("class", (), ("class", "red", ()))).render(&mut s);
    assert_eq!(s, " class=\"red\">");
}

#[test]
fn empty_between() {
    let mut s = String::new();
    Attributes(("class", "yellow", ("class", (), ("class", "red", ())))).render(&mut s);
    assert_eq!(s, " class=\"red yellow\">");
}

#[test]
fn empty_between_skip() {
    let mut s = String::new();
    Attributes(("class", "yellow", ("id", (), ("class", "red", ())))).render(&mut s);
    assert_eq!(s, " class=\"red yellow\">");
}

#[test]
fn bool_true() {
    let mut s = String::new();
    Attributes(("checked", true, ())).render(&mut s);
    assert_eq!(s, " checked>");
}

#[test]
fn bool_false() {
    let mut s = String::new();
    Attributes(("checked", false, ())).render(&mut s);
    assert_eq!(s, ">");
}

#[test]
fn bool_true_true() {
    let mut s = String::new();
    Attributes(("checked", true, ("checked", true, ()))).render(&mut s);
    assert_eq!(s, " checked>");
}

#[test]
fn bool_true_false() {
    let mut s = String::new();
    Attributes(("checked", false, ("checked", true, ()))).render(&mut s);
    assert_eq!(s, " checked>");
}

#[test]
fn bool_false_true() {
    let mut s = String::new();
    Attributes(("checked", true, ("checked", false, ()))).render(&mut s);
    assert_eq!(s, " checked>");
}

#[test]
fn bool_false_false() {
    let mut s = String::new();
    Attributes(("checked", false, ("checked", false, ()))).render(&mut s);
    assert_eq!(s, ">");
}

#[test]
fn bool_true_false_true() {
    let mut s = String::new();
    Attributes(("checked", true, ("checked", false, ("checked", true, ())))).render(&mut s);
    assert_eq!(s, " checked>");
}
