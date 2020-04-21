#![allow(non_snake_case)]

mod tests {
    use DED_backend::handlers::workout;
    use DED_backend::models::workouts::{Workout, NewWorkout, CompleteWorkout, WorkoutList};
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
            name: "Friday morning workout".to_string(),
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
            name: "working hard".to_string(),
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

    #[actix_rt::test]
    async fn test_workout_delete() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();
        let mut app = test::init_service(
            App::new()
                .route("/workouts/{id}/", web::delete().to(workout::delete))
        )
        .await;

        let test_find = NewWorkout {
            origin_id: 66,
            exercise: 4,
            name: "working hard".to_string(),
            description: "it has a nice flaky crust and good flavor.".to_string(),
            notes: "F".to_string(),
        };

        let test_find_results = test_find.create(&conn).unwrap();

        let req = test::TestRequest::delete()
                  .header(header::CONTENT_TYPE, "application/json")
                  .uri(format!("/workouts/{}/", test_find_results.id).as_str())
                  .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_workout_update() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();
        let mut app = test::init_service(
            App::new()
                .route("/workouts/{id}/", web::get().to(workout::find_by_id))
                .route("/workouts/{id}/", web::delete().to(workout::delete))
        )
        .await;

        let test1 = NewWorkout {
            origin_id: 66,
            exercise: 4,
            name: "working hard".to_string(),
            description: "it has a nice flaky crust and good flavor.".to_string(),
            notes: "F".to_string(),
        };

        let test2 = NewWorkout {
            origin_id: 666,
            exercise: 44,
            name: "working harder".to_string(),
            description: "it has a nice flaky crust and good flavor.".to_string(),
            notes: "FF".to_string(),
        };

        let test_find_results = test1.create(&conn).unwrap();

        let req = test::TestRequest::patch()
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("/workouts/{}/", test_find_results.id).as_str())
            .set_payload(serde_json::to_string(&test2).unwrap())
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
                assert!(new_resp == test2);
            }
        }
    }

    #[actix_rt::test]
    async fn test_workout_complete_without_notes() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();

        let mut app = test::init_service(
            App::new()
                .route("/workouts/complete/{id}/", web::post().to(workout::complete))
                .route("/worksets/{id}/", web::get().to(workout::find_by_id))
        )
        .await;

        let test1 = NewWorkout {
            origin_id: 101,
            exercise: 55,
            name: "working out harder".to_string(),
            description: "sweaty mcsweatface".to_string(),
            notes: "B#".to_string(),
        };

        let test_create_results = test1.create(&conn).unwrap();

        let req = test::TestRequest::post()
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("/workouts/complete/{}/", test_create_results.id).as_str())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        match resp.status().is_success() {
            false => (),
            true => {
                let n_req = test::TestRequest::get()
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(format!("/worksets/{}/", test_create_results.id).as_str())
                    .to_request();
                let new_resp: Workout = test::read_response_json(&mut app, n_req).await;
                assert!(new_resp == test1);
            }
        }
    }

    #[actix_rt::test]
    async fn test_workout_complete_with_notes() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();

        let mut app = test::init_service(
            App::new()
                .route("/workouts/complete/{id}/", web::post().to(workout::complete))
                .route("/worksets/{id}/", web::get().to(workout::find_by_id))
        )
        .await;

        let test1 = NewWorkout {
            origin_id: 101,
            exercise: 55,
            name: "working out harder".to_string(),
            description: "sweaty mcsweatface".to_string(),
            notes: "B#".to_string(),
        };

        let test2 = CompleteWorkout {
            notes: Some("C#".to_string())
        };

        let test_create_results = test1.create(&conn).unwrap();

        let req = test::TestRequest::post()
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("/workouts/complete/{}/", test_create_results.id).as_str())
            .set_payload(serde_json::to_string(&test2).unwrap())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        match resp.status().is_success() {
            false => (),
            true => {
                let n_req = test::TestRequest::get()
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(format!("/worksets/{}/", test_create_results.id).as_str())
                    .to_request();
                let new_resp: Workout = test::read_response_json(&mut app, n_req).await;
                assert_eq!(new_resp.notes, test2.notes.unwrap());
            }
        }
    }

    #[actix_rt::test]
    async fn test_workout_get_by_origin_id() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();

        let mut app = test::init_service(
            App::new()
                .route("/workouts/find_by_origin/{id}/", web::get().to(workout::find_by_origin_id))
        )
        .await;

        let test1 = NewWorkout {
            origin_id: 555,
            exercise: 55,
            name: "working out harder".to_string(),
            description: "sweaty mcsweatface".to_string(),
            notes: "B#".to_string(),
        };

        let test2 = NewWorkout {
            origin_id: 555,
            exercise: 65,
            name: "working out harder".to_string(),
            description: "sweaty mcsweatface".to_string(),
            notes: "B#".to_string(),
        };

        let _test1_create = test1.create(&conn).unwrap();
        let _test2_create = test2.create(&conn).unwrap();

        let req = test::TestRequest::get()
            .header(header::CONTENT_TYPE, "application/json")
            .uri("/workouts/find_by_origin_id/555/")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        match resp.status().is_success() {
            false => (),
            true => {
                let n_req = test::TestRequest::get()
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri("/workouts/find_by_origin_id/555/")
                    .to_request();
                let new_resp: WorkoutList = test::read_response_json(&mut app, n_req).await;
                assert!(new_resp.0.len() >= 2);
            }
        }
    }
}

