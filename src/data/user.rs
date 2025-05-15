use std::collections::HashMap;

use super::entity::{FullTextEntities, UrlEntity};
use super::withheld::Withheld;
use crate::id::NumericId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::OffsetDateTime;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserUrlEntites {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlEntity>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "arbitrary_precision", derive(Eq))]
pub struct UserEntities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<UserUrlEntites>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<FullTextEntities>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserPublicMetrics {
    pub followers_count: usize,
    pub following_count: usize,
    pub tweet_count: usize,
    pub listed_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "arbitrary_precision", derive(Eq))]
pub struct User {
    pub id: NumericId,
    pub name: String,
    pub username: String,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<UserEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_tweet_id: Option<NumericId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<UserPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withheld: Option<Withheld>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parody: Option<bool>,
    #[serde(
        default,
        with = "option_usize_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub verified_followers_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
}

mod option_usize_string {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => s
                .parse::<usize>()
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }

    pub fn serialize<S>(value: &Option<usize>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_some(&v.to_string()),
            None => serializer.serialize_none(),
        }
    }
}
