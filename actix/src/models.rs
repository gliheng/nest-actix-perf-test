use crate::schema::{users, posts};
use serde::Serialize;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}


#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub published: bool,
    pub author_id: i32,
}

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub author_id: i32,
}