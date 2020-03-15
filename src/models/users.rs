use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fname: String,
    pub email: String,
    pub salt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub id: i32,
    pub username: String,
    pub fname: String,
    pub email: String,
    pub salt: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

impl User {
    //pub fn get_user<'a>(id: &'a i32) -> Result<User, diesel::result::Error> {
    //    use crate::schema::users;
    //    users::table.find(id).get_result::<User>(conn)
    //}

    pub fn create(&self) -> Result<User, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::establish_connection;

       let connection = establish_connection();
        diesel::insert_into(users::table)
            .values(self)
            .get_result(&connection)
    }
}


//impl UserList {
//    pub fn get_users(conn: &PgConnection) -> UserList {
//
//        let results = users
//            .load::<User>(conn)
//            .expect("Error retreiving users");
//
//        UserList(results)
//    }
//}
