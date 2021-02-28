#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Password {
    pub id: i64,
    pub website: String,
    pub password: String,
    pub user_id: i32,
}
