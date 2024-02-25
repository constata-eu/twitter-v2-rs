use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct NoteTweet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
