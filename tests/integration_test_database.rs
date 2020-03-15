extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::{PgConnection, Pg};
use dotenv::dotenv;
use std::env;


mod tests {
    use DED_backend::establish_connection;
    use DED_backend::models::{User, NewUser};
    use diesel::result::Error;

    #[test]
    fn test_db_connection() {
        let conn = establish_connection("DATABASE_TEST_URL");
    }
}