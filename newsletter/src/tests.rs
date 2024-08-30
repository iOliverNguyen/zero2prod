use actix_web::{http::StatusCode, test};
use newsletter::create_app;

#[actix_web::test]
async fn test_health_ok() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
