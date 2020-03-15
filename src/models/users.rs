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
