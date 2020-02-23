#[cfg(test)]

mod tests {
    use DED_backend::{index_with_name,index};
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_index_with_name_get() {
        let mut app = test::init_service(
            App::new()
                .service(index_with_name)
        )
        .await;
        let req = test::TestRequest::with_header("content-type", "text/plain")
                  .uri("/bryce/index.html")
                  .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(
            App::new()
                .service(index)
        )
        .await;
        let req = test::TestRequest::with_header("content-type", "text/plain")
                  .uri("/")
                  .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
