use serde::{Serialize, Deserialize};
#[cfg(test)]

#[cfg(test)]
use mockall::predicate::*;

use crate::schema::users;
use diesel::PgConnection;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable,
    AsChangeset, QueryableByName, Serialize, Deserialize
)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fname: String,
    pub email: String,
    pub passwd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub fname: String,
    pub email: String,
    pub passwd: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserList( pub Vec<User> );

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: i32,
    pub email: String,
    pub fname: String,
    pub username: String
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            id: user.id,
            email: user.email,
            fname: user.fname,
            username: user.username
        }
    }
}

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

impl PartialEq<NewUser> for User {
    fn eq(&self, other:& NewUser) -> bool {
        self.username == other.username &&
        self.fname == other.fname &&
        self.email == other.email
    }
}

impl PartialEq<NewUser> for SlimUser {
    fn eq(&self, other:& NewUser) -> bool {
        self.username == other.username &&
        self.fname == other.fname &&
        self.email == other.email
    }
}

impl PartialEq<User> for NewUser {
    fn eq(&self, other:& User) -> bool {
        self.username == other.username &&
        self.fname == other.fname &&
        self.email == other.email
    }
}

impl PartialEq<User> for SlimUser {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id &&
        self.username == other.username &&
        self.fname == other.fname &&
        self.email == other.email
    }
}

impl PartialEq<SlimUser> for User {
    fn eq(&self, other: &SlimUser) -> bool {
        self.id == other.id &&
        self.username == other.username &&
        self.fname == other.fname &&
        self.email == other.email
    }
}

impl PartialEq<SlimUser> for NewUser {
    fn eq(&self, other:& SlimUser) -> bool {
        self.username == other.username &&
        self.fname == other.fname &&
        self.email == other.email
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
        let t_passwd = "some_like_MSG";

        let t_user = User {
            id: t_id,
            username: t_uname.to_string(),
            fname: t_fname.to_string(),
            email: t_email.to_string(),
            passwd: t_passwd.to_string()
        };

        assert_eq!(t_id, t_user.id);
        assert_eq!(t_uname, t_user.username);
        assert_eq!(t_fname, t_user.fname);
        assert_eq!(t_email, t_user.email);
        assert_eq!(t_passwd, t_user.passwd);

    }
}
