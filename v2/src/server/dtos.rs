use serde::Serialize;


#[derive(Serialize)]
pub struct TagDto {
    pub name: String,
    pub id: String,
    pub marker_count: i64,       
}