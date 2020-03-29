use serde::{Serialize, Deserialize};
#[cfg(test)]

#[cfg(test)]
use mockall::predicate::*;

use crate::schema::users;
use diesel::PgConnection;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fname: String,
    pub email: String,
    pub passwd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub fname: String,
    pub email: String,
    pub passwd: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserList( pub Vec<User> );


impl User {
    pub fn get_user (id: i32, conn: &PgConnection) -> Result<User, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        users::table.find(id).get_result(conn)
    }
}

impl NewUser {
    pub fn create(&self, connection: &PgConnection) -> Result<User, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(users::table)
            .values(self)
            .get_result(connection)
    }
}


impl UserList {
    pub fn get_users(conn: &PgConnection) -> Self {

        use diesel::RunQueryDsl;
        use crate::schema::users::dsl::*;

        let results = users
            .load::<User>(conn)
            .expect("Error retreiving users");

        UserList(results)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_structure() {

        let t_id = 1999;
        let t_uname = "TestUser";
        let t_fname = "Usable User";
        let t_email = "testuser@testdomain.com";
        let t_salt = "some_like_MSG";

        let t_user = User {
            id: t_id,
            username: t_uname.to_string(),
            fname: t_fname.to_string(),
            email: t_email.to_string(),
            salt: t_salt.to_string()
        };

        assert_eq!(t_id, t_user.id);
        assert_eq!(t_uname, t_user.username);
        assert_eq!(t_fname, t_user.fname);
        assert_eq!(t_email, t_user.email);
        assert_eq!(t_salt, t_user.salt);

    }
}
