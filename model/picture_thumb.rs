#[derive(Serialize, Queryable)]
pub struct PictureThumb {
    pub id: i32,
    pub thumb: String,
}