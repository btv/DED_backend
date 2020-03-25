#![allow(non_snake_case)]

mod tests {
    use DED_backend::handlers::set;
    use DED_backend::models::sets::Set;
    use actix_web::{web, test, App, http::StatusCode, http::header};

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
            \"description\": \"none\"
        }";

        let req = test::TestRequest::post()
                  .header(header::CONTENT_TYPE, "application/json")
                  .uri("/sets/new/")
                  .set_payload(payload)
                  .to_request();

        let resp_set: Set = test::read_response_json(&mut app, req).await;
        assert_eq!(resp_set.completed_reps, 0);
        assert_eq!(resp_set.completed_value, "");
        assert_eq!(resp_set.style, "none");
        assert_eq!(resp_set.unit, "none");
        assert_eq!(resp_set.goal_reps, 10);
    }

    #[actix_rt::test]
    async fn test_set_post_delete() {
        let mut app = test::init_service(
            App::new()
                .route("/sets/delete/{id}/", web::post().to(set::delete))
        )
        .await;

        let req = test::TestRequest::post()
                  .header(header::CONTENT_TYPE, "application/json")
                  .uri("/sets/delete/1/")
                  .to_request();


        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_set_delete_delete() {
        let mut app = test::init_service(
            App::new()
                .route("/sets/delete/{id}/", web::delete().to(set::delete))
        )
        .await;

        let req = test::TestRequest::delete()
                  .header(header::CONTENT_TYPE, "application/json")
                  .uri("/sets/delete/2/")
                  .to_request();


        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
