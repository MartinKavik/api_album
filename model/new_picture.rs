use crate::picture_sch::*;

#[derive(Insertable)]
#[table_name="picture"]
pub struct NewPicture {
    pub data: String,
}