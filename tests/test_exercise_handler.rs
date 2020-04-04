#![allow(non_snake_case)]

mod tests {
    use DED_backend::handlers::exercise;
    use DED_backend::models::exercises::{Exercise,NewExercise};
    use actix_web::{web, test, App, http::StatusCode, http::header};

    #[actix_rt::test]
    async fn test_exercise_create() {

        let mut app = test::init_service(
            App::new()
                .route(
                    "/exercises/create/",
                    web::post().to(exercise::create_exercise)
                )
        )
        .await;

        let new_ex = NewExercise {
            origin_id: 100,
            set_id: 69,
            fname: "Something special".to_string(),
            exercise_type: 5,
            description: "I have no idea what to put here.".to_string(),
            notes: "C#".to_string()
        };

        let req = test::TestRequest::post()
                  .header(header::CONTENT_TYPE, "application/json")
                  .uri("/exercises/create/")
                  .set_payload(
                      serde_json::to_string(&new_ex).unwrap()
                  )
                  .to_request();

        let resp_ex: Exercise = test::read_response_json(&mut app, req).await;
        assert!(resp_ex == new_ex);
    }
}
