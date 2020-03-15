use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fname: String,
    pub email: String,
    pub salt: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a>{
    pub id: &'a i32,
    pub username: &'a str,
    pub fname: &'a str,
    pub email: &'a str,
    pub salt: &'a str,
}

impl User {
    pub fn get_user<'a>(conn: &PgConnection, id: &'a i32) -> Result<User, diesel::result::Error> {
        use schema::users;
        users::table.find(id).get_result::<User>(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

impl UserList {
    pub fn get_users(conn: &PgConnection) -> UserList {
        use crate::schema::users::dsl::*;


        let results = users
            .load::<User>(conn)
            .expect("Error retreiving users");

        UserList(results)
    }
}
