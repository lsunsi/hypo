impl axum::response::IntoResponse for crate::Raw {
    fn into_response(self) -> axum::response::Response<axum::body::Body> {
        axum::response::Html(self.0).into_response()
    }
}
