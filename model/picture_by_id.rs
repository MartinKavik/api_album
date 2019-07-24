#[derive(Serialize, Queryable)]
pub struct PictureById {
    pub id: i32,
    pub data: String,
    pub model: Option<String>,
    pub date: String,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}