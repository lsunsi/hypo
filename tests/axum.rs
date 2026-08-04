#[test]
#[cfg(feature = "axum")]
fn into_response() {
    use axum::response::IntoResponse;
    use hypo::{div, render};

    let response = IntoResponse::into_response(render(div!(id = "main", "oiblz")));
    let (parts, body) = response.into_parts();

    assert_eq!(parts.status, 200);
    assert_eq!(
        parts.headers.get("content-type").unwrap(),
        "text/html; charset=utf-8"
    );

    let mut body = Box::pin(axum::body::to_bytes(body, usize::MAX));
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());

    let body = loop {
        match body.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(a) => break a.unwrap(),
            std::task::Poll::Pending => {}
        }
    };

    assert_eq!(
        String::from_utf8(body.to_vec()).unwrap(),
        "<div id=\"main\">oiblz</div>"
    );
}
