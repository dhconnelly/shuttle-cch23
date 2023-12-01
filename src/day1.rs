use axum::{extract::Path, response::IntoResponse, routing::get, Router};

async fn part1(Path((num1, num2)): Path<(i64, i64)>) -> impl IntoResponse {
    (num1 ^ num2).pow(3).to_string()
}

pub fn router() -> Router {
    Router::new().route("/:num1/:num2", get(part1))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_task1() {
        let app = router();
        let req = Request::builder().uri("/4/8").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        let data = res.into_body().collect().await.unwrap().to_bytes();
        let answer = String::from_utf8(data.to_vec()).unwrap();
        assert_eq!(answer, "1728");
    }
}
