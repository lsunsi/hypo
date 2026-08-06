macro_rules! assert_render {
    ($hypo:expr, $ex:expr) => {{
        assert_eq!(hypo::render($hypo).0, $ex);
    }};
}

#[test]
fn char() {
    assert_render!('!', "!");
    assert_render!('<', "&lt;");
    assert_render!('>', "&gt;");
    assert_render!('&', "&amp;");
    assert_render!('"', "&quot;");
    assert_render!('\'', "&apos;");
}

#[test]
fn str() {
    assert_render!("oiblz", "oiblz");
    assert_render!(
        "<p>'\"bl&z\"'</p>",
        "&lt;p&gt;&apos;&quot;bl&amp;z&quot;&apos;&lt;/p&gt;"
    );
}

#[test]
fn borrow_string() {
    assert_render!(&String::from("oiblz"), "oiblz");
    assert_render!(
        &String::from("<p>'\"bl&z\"'</p>"),
        "&lt;p&gt;&apos;&quot;bl&amp;z&quot;&apos;&lt;/p&gt;"
    );
}

#[test]
fn string() {
    assert_render!(String::from("oiblz"), "oiblz");
    assert_render!(
        String::from("<p>'\"bl&z\"'</p>"),
        "&lt;p&gt;&apos;&quot;bl&amp;z&quot;&apos;&lt;/p&gt;"
    );
}

#[test]
fn numbers() {
    assert_render!(0u8, "0");
    assert_render!(1i8, "1");
    assert_render!(2u16, "2");
    assert_render!(3i16, "3");
    assert_render!(4u32, "4");
    assert_render!(5i32, "5");
    assert_render!(6u64, "6");
    assert_render!(7i64, "7");
    assert_render!(8u128, "8");
    assert_render!(9i128, "9");
    assert_render!(0usize, "0");
    assert_render!(1isize, "1");
}

#[test]
fn option() {
    assert_render!(None::<&str>, "");
    assert_render!(Some("oiblz"), "oiblz");
}

#[test]
fn result() {
    assert_render!(Err::<&str, _>("oi"), "oi");
    assert_render!(Ok::<_, &str>("blz"), "blz");
}

#[test]
fn array() {
    assert_render!(["oi", "blz"], "oiblz");
}

#[test]
fn vec() {
    assert_render!(vec!["oi", "blz"], "oiblz");
}

#[test]
fn map() {
    assert_render!((0..3).map(|i| i + 1), "123");
}

#[test]
fn tuples() {
    assert_render!((), "");
    assert_render!(("0",), "0");
    assert_render!(("0", "1"), "01");
    assert_render!(("0", "1", "2"), "012");
    assert_render!(("0", "1", "2", "3"), "0123");
    assert_render!(("0", "1", "2", "3", "4"), "01234");
    assert_render!(("0", "1", "2", "3", "4", "5"), "012345");
    assert_render!(("0", "1", "2", "3", "4", "5", "6"), "0123456");
    assert_render!(("0", "1", "2", "3", "4", "5", "6", "7"), "01234567");
    assert_render!(
        (
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"
        ),
        "0123456789101112131415"
    );
}

#[test]
fn raw() {
    use hypo::Raw;
    assert_render!(Raw("<p>oiblz</p>"), "<p>oiblz</p>");
}
