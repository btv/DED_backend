mod tests {
    use DED_backend::handlers::index::{index};
    use actix_web::{web, test, App};

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(
            App::new()
                .route("/", web::get().to(index))
        )
        .await;
        let req = test::TestRequest::with_header("content-type", "text/plain")
                  .uri("/")
                  .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
