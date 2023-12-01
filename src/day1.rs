use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn part1(Path(nums): Path<String>) -> Result<String, StatusCode> {
    let nums = nums
        .split('/')
        .map(|num| i64::from_str_radix(num, 10))
        .collect::<Result<Vec<i64>, _>>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let xor =
        nums.into_iter().reduce(|x, y| x ^ y).ok_or(StatusCode::BAD_REQUEST)?;
    Ok(xor.pow(3).to_string())
}

pub fn router() -> Router {
    Router::new().route("/*nums", get(part1))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn request(uri: &str) -> i64 {
        let app = router();
        let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        let data = res.into_body().collect().await.unwrap().to_bytes();
        let answer = String::from_utf8(data.to_vec()).unwrap();
        i64::from_str_radix(&answer, 10).unwrap()
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
