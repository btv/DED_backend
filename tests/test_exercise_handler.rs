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
            name: "Something special".to_string(),
            exercise_type: 5,
            description: "I have no idea what to put here.".to_string(),
            notes: "C#".to_string()
        };

        let req = test::TestRequest::post()
                  .header(header::CONTENT_TYPE, "application/json")
                  .uri("/exercises/create/")
                  .set_payload(serde_json::to_string(&new_ex).unwrap())
                  .to_request();

        let resp_ex: Exercise = test::read_response_json(&mut app, req).await;
        assert!(resp_ex == new_ex);
    }

    #[actix_rt::test]
    async fn test_exercise_delete() {
        let mut app = test::init_service(
            App::new()
                .route("/exercise/{id}/", web::delete().to(exercise::delete))
        )
        .await;

        let req = test::TestRequest::delete()
                  .header(header::CONTENT_TYPE, "application/json")
                  .uri("/exercise/1/")
                  .to_request();


        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_find_by_id() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();

        let mut app = test::init_service(
            App::new()
                .route("/exercises/{id}/", web::get().to(exercise::find_by_id))
        )
        .await;

        let test_find = NewExercise {
            origin_id: 55,
            set_id: 56,
            name: "Working hard".to_string(),
            exercise_type: 3,
            description: "describe what it is you do here.".to_string(),
            notes: "B#".to_string(),
        };

        let test_find_results = test_find.create(&conn).unwrap();

        let req = test::TestRequest::get()
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("/exercises/{}/", test_find_results.id).as_str())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        match resp.status().is_success() {
            false => (),
            true => {
                let n_req = test::TestRequest::get()
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(format!("/exercises/{}/", test_find_results.id).as_str())
                    .to_request();

                let new_resp: Exercise = test::read_response_json(&mut app, n_req).await;
                assert!(new_resp == test_find);
            }
        }
    }

    #[actix_rt::test]
    async fn test_update_by_set_id() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();
        let mut app = test::init_service(
            App::new()
                .route("/exercises/{id}/", web::patch().to(exercise::update_by_id))
                .route("/exercises/{id}/", web::get().to(exercise::find_by_id))
        )
        .await;

        let test1 = NewExercise {
            origin_id: 55,
            set_id: 56,
            name: "Working hard".to_string(),
            exercise_type: 3,
            description: "describe what it is you do here.".to_string(),
            notes: "B#".to_string(),
        };

        let test2 = NewExercise {
            origin_id: 555,
            set_id: 566,
            name: "Working harder".to_string(),
            exercise_type: 3,
            description: "still describing what I do here.".to_string(),
            notes: "B##".to_string(),
        };

        let test_find_results = test1.create(&conn).unwrap();

        let req = test::TestRequest::patch()
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("/exercises/{}/", test_find_results.id).as_str())
            .set_payload(serde_json::to_string(&test2).unwrap())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        match resp.status().is_success() {
            false => (),
            true => {
                let n_req = test::TestRequest::get()
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(format!("/exercises/{}/", test_find_results.id).as_str())
                    .to_request();
                let new_resp: Exercise = test::read_response_json(&mut app, n_req).await;
                assert!(new_resp == test2);
            }
        }
    }
}
