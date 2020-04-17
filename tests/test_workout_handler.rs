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

    #[actix_rt::test]
    async fn test_find_by_workout_id() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();

        let mut app = test::init_service(
            App::new()
                .route("/workouts/{id}/", web::get().to(workout::find_by_id))
        )
        .await;

        let test_find = NewWorkout {
            origin_id: 66,
            exercise: 4,
            fname: "working hard".to_string(),
            description: "it has a nice flaky crust and good flavor.".to_string(),
            notes: "F".to_string(),
        };

        let test_find_results = test_find.create(&conn).unwrap();

        let req = test::TestRequest::get()
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("/workouts/{}/", test_find_results.id).as_str())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        match resp.status().is_success() {
            false => (),
            true => {
                let n_req = test::TestRequest::get()
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(format!("/workouts/{}/", test_find_results.id).as_str())
                    .to_request();
                let new_resp: Workout = test::read_response_json(&mut app, n_req).await;
                assert!(new_resp == test_find);
            }
        }
    }
}

