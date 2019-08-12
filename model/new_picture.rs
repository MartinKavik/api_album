use crate::picture_sch::*;

#[derive(Insertable)]
#[table_name="picture"]
pub struct NewPicture {
    pub data: String,
    pub model: Option<String>,
    pub date: std::time::SystemTime,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub thumb: String,
}