#![allow(non_snake_case)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::{PgConnection, Pg};
use dotenv::dotenv;
use std::env;
use diesel::NotFound;


mod tests {
    use DED_backend::{establish_connection, create_user};
    use DED_backend::models::{User, NewUser};
    use diesel::result::Error;
    use diesel::RunQueryDsl;
    use actix_web::http::header::q;

    #[test]
    fn test_db_insert_and_find() {
        use DED_backend::schema::users;

        let conn = establish_connection("DATABASE_TEST_URL");

        let xxx = diesel::delete(DED_backend::schema::users::dsl::users)
            .execute(&conn);

        // assert_eq!(xxx,Ok(0));

        let t_id = 1999;
        let t_uname = "TestUser";
        let t_fname = "Usable User";
        let t_email = "testuser@testdomain.com";
        let t_salt = "some_like_MSG";

        let t_user = create_user(&conn, &t_id, &t_uname, &t_fname, &t_email, &t_salt);

        assert_eq!(t_id, t_user.id);
        assert_eq!(t_uname, t_user.username);
        assert_eq!(t_fname, t_user.fname);
        assert_eq!(t_email, t_user.email);
        assert_eq!(t_salt, t_user.salt);

        let user_result = User::get_user(&conn, &t_id);

        match user_result{
            Ok(q_user) => {
                assert_eq!(t_id, q_user.id);
                assert_eq!(t_uname, q_user.username);
                assert_eq!(t_fname, q_user.fname);
                assert_eq!(t_email, q_user.email);
                assert_eq!(t_salt, q_user.salt);
            }
            Err(e) => {
                assert_eq!(5,2);
            }
        }

    }
    #[test]
    fn test_db_not_found() {
        use DED_backend::schema::users;

        let conn = establish_connection("DATABASE_TEST_URL");
        let t_id = -123456;

        let user_result = User::get_user(&conn, &t_id);

        match user_result{
            Ok(q_user) => {
                assert_eq!(t_id, q_user.id);
            }
            Err(e) => {
                assert_eq!(e,diesel::NotFound);
            }
        }
    }
}