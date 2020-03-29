#![allow(non_snake_case)]

mod tests {
    use DED_backend::handlers::set;
    use DED_backend::models::sets::{Set, NewSet};
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

    #[actix_rt::test]
    async fn test_find_by_exercise_id() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();

        let mut app = test::init_service(
            App::new()
                .route("/sets/find_by_exercise_id/{ex_id}/", web::get().to(set::find_by_exercise_id))
        )
        .await;

        let test_find = NewSet {
            exercise_id: -1,
            style: "gangam".to_string(),
            unit: "none".to_string(),
            goal_reps: 10,
            goal_value: "none".to_string(),
            description: "something".to_string(),
        };

        test_find.create(&conn);

        let req = test::TestRequest::get()
            .header(header::CONTENT_TYPE, "application/json")
            .uri("/sets/find_by_exercise_id/-1/")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_find_by_set_id() {
        use DED_backend::establish_connection;

        let conn = establish_connection().get().unwrap();

        let mut app = test::init_service(
            App::new()
                .route("/sets/find_by_set_id/{id}/", web::get().to(set::find_by_set_id))
        )
        .await;

        let test_find = NewSet {
            exercise_id: -1,
            style: "gangam".to_string(),
            unit: "none".to_string(),
            goal_reps: 10,
            goal_value: "none".to_string(),
            description: "something".to_string(),
        };

        let test_find_results = test_find.create(&conn).unwrap();

        let req = test::TestRequest::get()
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("/sets/find_by_set_id/{}/", test_find_results.id).as_str())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        match resp.status().is_success() {
            false => assert_eq!(1,5),
            true => {
                let n_req = test::TestRequest::get()
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(format!("/sets/find_by_set_id/{}/", test_find_results.id).as_str())
                    .to_request();
                let new_resp: Set = test::read_response_json(&mut app, n_req).await;
                assert!(new_resp == test_find);
            }
        }
    }
}

