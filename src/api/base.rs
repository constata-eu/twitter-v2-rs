use crate::api_result::{ApiPayload, ApiResponse, ApiResponseExt, ApiResult};
use crate::authorization::Authorization;
use crate::error::Result;
use crate::utils::JsonStream;
use crate::Error;
use futures::prelude::*;
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, IntoUrl, Method, Url};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use serde_path_to_error::deserialize;

#[derive(Debug)]
pub struct TwitterApi<A> {
    client: Client,
    base_url: Url,
    auth: Arc<A>,
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn new(auth: A) -> Self {
        Self {
            client: Client::builder().pool_max_idle_per_host(0).build().unwrap(),
            base_url: Url::parse("https://api.twitter.com/2/").unwrap(),
            auth: Arc::new(auth),
        }
    }

    pub fn auth(&self) -> &A {
        &self.auth
    }

    pub(crate) fn url(&self, url: impl AsRef<str>) -> Result<Url> {
        Ok(self.base_url.join(url.as_ref())?)
    }

    pub(crate) fn request(&self, method: Method, url: impl IntoUrl) -> reqwest::RequestBuilder {
        self.client.request(method, url)
    }

    pub(crate) async fn send<T: DeserializeOwned, M: DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> ApiResult<A, T, M> {
        let mut req = req.build()?;
        let authorization = self.auth.header(&req).await?;
        let _ = req.headers_mut().insert(AUTHORIZATION, authorization);
        let url = req.url().clone();

        let raw_response = self
            .client
            .execute(req)
            .await?
            .api_error_for_status()
            .await?;

        let text = raw_response.text().await?;

        let mut deserializer = serde_json::Deserializer::from_str(&text);
        let response = deserialize(&mut deserializer)
            .map_err(|e| Error::custom(format!("Error parsing twitter response: {e} | url: {url} | response: {text}")))?;
        Ok(ApiResponse::new(self, url, response))
    }

    pub(crate) async fn stream<T: DeserializeOwned, M: DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<impl Stream<Item = Result<ApiPayload<T, M>>>> {
        let mut req = req.build()?;
        let authorization = self.auth.header(&req).await?;
        let _ = req.headers_mut().insert(AUTHORIZATION, authorization);
        Ok(JsonStream::new(
            self.client
                .execute(req)
                .await?
                .api_error_for_status()
                .await?
                .bytes_stream(),
        ))
    }
}

impl<A> Clone for TwitterApi<A> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            auth: self.auth.clone(),
        }
    }
}
