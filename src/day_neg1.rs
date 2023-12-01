use axum::{http::StatusCode, routing::get, Router};

async fn internal_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn router() -> Router {
    Router::new().route("/error", get(internal_error))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test() {
        let app = router();
        let req = Request::builder()
            .uri("/error")
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
