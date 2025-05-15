use crate::id::NumericId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Metric {
    app_install_attempts: usize,
    app_opens: usize,
    detail_expands: usize,
    email_tweet: usize,
    engagements: usize,
    follows: usize,
    hashtag_clicks: usize,
    impressions: usize,
    likes: usize,
    link_clicks: usize,
    media_engagements: usize,
    media_views: usize,
    permalink_clicks: usize,
    profile_visits: usize,
    quote_tweets: usize,
    replies: usize,
    retweets: usize,
    url_clicks: usize,
    user_profile_clicks: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TimestampedMetric {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    pub metrics: Vec<Metric>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analytics {
    pub id: NumericId,
    pub timestamped_metrics: Vec<TimestampedMetric>,
}
