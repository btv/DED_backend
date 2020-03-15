#![allow(non_snake_case)]
#[macro_use]
extern crate diesel;
extern crate dotenv;


mod tests {
    use DED_backend::establish_connection;
    use DED_backend::models::users::User;

    use diesel::RunQueryDsl;


    #[test]
    fn test_user_insert() {
    }

    #[test]
    fn test_user_find() {
    }
}
