#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String
}