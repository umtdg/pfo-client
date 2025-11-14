use std::fmt::{Debug, Display};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProblemDetail {
    #[serde(rename = "type")]
    _type: Option<String>,
    title: Option<String>,
    status: u16,
    detail: Option<String>,
    instance: Option<String>,
    #[serde(rename = "responseBody")]
    response_body: Option<String>,
}

impl Debug for ProblemDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ProblemDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status)?;
        if let Some(title) = &self.title {
            write!(f, " {}", title)?;
        }

        if let Some(instance) = &self.instance {
            write!(f, " - {}", instance)?;
        }

        if let Some(detail) = &self.detail {
            write!(f, " - {}", detail)?;
        }

        if let Some(response_body) = &self.response_body {
            write!(f, "\n{}", response_body)?;
        }

        Ok(())
    }
}
