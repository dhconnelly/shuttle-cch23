use crate::extractors::PathVec;
use axum::{http::StatusCode, routing::get, Router};

async fn solve(PathVec(nums): PathVec<i64>) -> Result<String, StatusCode> {
    let answer = nums.into_iter().reduce(|x, y| x ^ y).unwrap_or(0).pow(3);
    Ok(answer.to_string())
}

pub fn router() -> Router {
    Router::new().route("/*nums", get(solve))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn request(uri: &str) -> i64 {
        let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
        let res = router().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let data = res.into_body().collect().await.unwrap().to_bytes();
        String::from_utf8(data.to_vec()).unwrap().parse().unwrap()
    }

    #[tokio::test]
    async fn test_task1() {
        let answer = request("/4/8").await;
        assert_eq!(answer, 1728);
    }

    #[tokio::test]
    async fn test_task2() {
        let answer = request("/4/5/8/10").await;
        assert_eq!(answer, 27);
    }
}
