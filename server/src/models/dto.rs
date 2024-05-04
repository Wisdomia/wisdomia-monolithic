use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateWisdomDTO {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWisdomDTO {
    pub id: i32,
    pub description: String,
}
