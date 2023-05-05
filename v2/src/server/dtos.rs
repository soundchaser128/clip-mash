use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TagDto {
    pub name: String,
    pub id: String,
    pub marker_count: i64,
}

#[derive(Serialize, Debug)]
pub struct PerformerDto {
    pub id: String,
    pub scene_count: i64,
    pub name: String,
    pub image_url: Option<String>,
    pub tags: Vec<String>,
    pub rating: Option<i64>,
    pub favorite: bool,
}
