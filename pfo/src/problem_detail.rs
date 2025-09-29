use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProblemDetail {
    #[serde(rename = "type")]
    pub ptype: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub instance: String,
}
