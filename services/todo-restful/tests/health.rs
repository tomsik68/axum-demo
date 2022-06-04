use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum_demo::app::create_app_with_spec;
use axum_demo::ioc::Container;
use axum_demo::ioc::TodoFactoryImpl;
use tower::ServiceExt;

#[tokio::test]
async fn test_health() {
    let testing_container = Container::builder()
        .todo_factory(TodoFactoryImpl::default())
        .build();
    let app = create_app_with_spec(testing_container);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"don't worry, I'm healthy");
}
