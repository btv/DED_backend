mod tests {
    use DED_backend::handlers::index;
    use DED_backend::models::users::{NewUser, SlimUser};
    use actix_web::{web, test, App, http::header};


    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(
            App::new()
                .route("/", web::get().to(index::index))
        )
        .await;
        let req = test::TestRequest::get()
                  .header(header::CONTENT_TYPE, "text/plain")
                  .uri("/")
                  .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_register() {
        let mut app = test::init_service(
            App::new()
                .route("/register/", web::post().to(index::register))
        )
        .await;

        let new_user = NewUser {
            username: "bob1984".to_string(),
            fname: "Bob Scratchit".to_string(),
            email: "bobscratchit1984@gmail.com".to_string(),
            passwd: "Sup3rS3cr3tP@ssw@ord".to_string()
        };

        let req = test::TestRequest::post()
                  .header(header::CONTENT_TYPE, "application/json")
                  .uri("/register/")
                  .set_payload(
                      serde_json::to_string(&new_user).unwrap()
                  )
                  .to_request();


        let resp_user: SlimUser = test::read_response_json(&mut app, req).await;
        assert!(resp_user == new_user);
    }
}
