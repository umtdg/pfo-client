use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProblemDetail {
    #[serde(rename = "type")]
    _type: String,
    title: String,
    status: u16,
    detail: String,
    instance: String,
}

impl Display for ProblemDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} - {} - {}",
            self.status, self.title, self.instance, self.detail
        )
    }
}
