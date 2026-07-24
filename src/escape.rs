pub fn str(mut raw: &str, to: &mut String) {
    while let Some(index) = raw.find(['\'', '"', '&', '<', '>']) {
        to.push_str(&raw[..index]);
        to.push_str(match raw.as_bytes()[index] {
            b'\'' => "&apos;",
            b'"' => "&quot;",
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            _ => "",
        });
        raw = &raw[index + 1..];
    }

    to.push_str(raw);
}

pub fn char(c: char, to: &mut String) {
    match c {
        '\'' => to.push_str("&apos;"),
        '"' => to.push_str("&quot;"),
        '&' => to.push_str("&amp;"),
        '<' => to.push_str("&lt;"),
        '>' => to.push_str("&gt;"),
        _ => to.push(c),
    }
}
