use axum::{routing::get, Router};
use cch23_dhconnelly::*;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

fn router() -> Router {
    Router::new().route("/", get(hello_world)).nest("/-1", day_neg1::router())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(router().into())
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test() {
        let app = router();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_nesting() {
        let app = router()
            .nest("/hello", Router::new().route("/world", get(hello_world)));
        let req =
            Request::builder().uri("/hello/world").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
