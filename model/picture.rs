#[derive(Serialize, Queryable)]
pub struct Picture {
    pub id: i32,
    pub data: String,
    pub model: Option<String>,
    pub date: std::time::SystemTime,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}