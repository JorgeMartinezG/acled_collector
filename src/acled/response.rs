use crate::acled::Incident;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub status: u8,
    pub success: bool,
    pub last_update: i32,
    pub count: u32,
    pub data: Vec<Incident>,
    pub filename: String,
}
