use axum::body::HttpBody;
use axum::http::Request;
use axum::http::StatusCode;
use axum_demo::api::RequestTodo;
use axum_demo::api::ResponseTodo;
use axum_demo::app::create_app_with_spec;
use axum_demo::ioc::Container;
use axum_demo::ioc::TodoFact;
use axum_demo::ioc::TodoFactoryImpl;
use axum_demo::todo_service::Todo;
use axum_demo::todo_service::TodoService;
use fake::Fake;
use fake::Faker;
use http::header::CONTENT_TYPE;
use hyper::Body;
use tower::util::ServiceExt;

#[tokio::test]
async fn create_todo() {
    let testing_container = Container::builder()
        .todo_factory(TodoFactoryImpl::default())
        .build();
    let app = create_app_with_spec(testing_container);
    let req: RequestTodo = Faker.fake();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/todos")
                .header(CONTENT_TYPE, "application/json")
                .method("POST")
                .body(serde_json::to_string(&req).unwrap().into())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let b = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let b: ResponseTodo = serde_json::from_slice(&b).unwrap();

    assert_eq!(b.text, req.text);
}

#[tokio::test]
async fn list_todos() {
    let todo_factory_impl = TodoFactoryImpl::default();

    todo_factory_impl
        .create()
        .add_todo("test".into())
        .await
        .unwrap();

    let testing_container = Container::builder().todo_factory(todo_factory_impl).build();
    let app = create_app_with_spec(testing_container);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/todos")
                .header(CONTENT_TYPE, "application/json")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let b = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let b: Vec<ResponseTodo> = serde_json::from_slice(&b).unwrap();

    assert_eq!(b.len(), 1);
    assert_eq!(b[0].text, "test".to_string());
}
