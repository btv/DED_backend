#![allow(non_snake_case)]

mod tests {
    use DED_backend::handlers::set;
    use actix_web::{web, test, App, http::StatusCode};

    #[actix_rt::test]
    async fn test_set_new() {
        let mut app = test::init_service(
            App::new()
                .route("/sets/new/", web::post().to(set::new))
        )
        .await;

        let payload = "{
            \"exercise_id\": -1,
            \"style\": \"none\",
            \"unit\": \"none\",
            \"goal_reps\": 10,
            \"goal_value\": \"none\",
            \"description\": \"none\",
            \"completed_reps\": 9,
            \"completed_value\": \"none\"
        }";

        let req = test::TestRequest::post()
                  .header("content-type", "application/json")
                  .uri("/sets/new/")
                  .set_payload(payload)
                  .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

}
