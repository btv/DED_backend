#![allow(non_snake_case)]

mod tests {
    use DED_backend::handlers::workout;
    use DED_backend::models::workouts::{Workout, NewWorkout};
    use actix_web::{web, test, App, http::StatusCode, http::header};

    #[actix_rt::test]
    async fn test_workout_new() {
        let mut app = test::init_service(
            App::new()
                .route("/workouts/new/", web::post().to(workout::create_workout))
        )
        .await;

        let payload = NewWorkout {
            origin_id: 55,
            exercise: 23,
            fname: "Friday morning workout".to_string(),
            description: "The workout I do friday morning when I wake up".to_string(),
            notes: "none".to_string(),
        };

        let req = test::TestRequest::post()
                  .header(header::CONTENT_TYPE, "application/json")
                  .uri("/workouts/new/")
                  .set_payload(serde_json::to_string(&payload).unwrap())
                  .to_request();

        let resp_wk: Workout = test::read_response_json(&mut app, req).await;
        assert!(resp_wk == payload);
    }
}

