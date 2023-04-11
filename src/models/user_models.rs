use crate::schema::user;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub hash: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "user"]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub hash: String,
}