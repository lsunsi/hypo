use hypo::Element;

#[test]
fn element() {
    let mut s = String::new();

    let mut el = Element::<false>::open(&mut s, "div");
    assert_eq!(&el as &str, "<div");

    el.push('>');
    assert_eq!(&el as &str, "<div>");

    Element::<true>::open(&mut el, "br").push('>');
    assert_eq!(&el as &str, "<div><br>");

    drop(el);
    assert_eq!(s, "<div><br></div>");
}
