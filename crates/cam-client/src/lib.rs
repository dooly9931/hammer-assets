//! CAM (Centralized Asset Management) Client
//!
//! This crate provides functionality for interacting with centralized asset management systems.

mod account;
mod instrument;
mod portfolio;
pub mod types;

use std::{env, sync::Arc};

use anyhow::{Result, anyhow};
use base64::prelude::*;
use hmac::{Hmac, Mac};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::{
    Client, Request, Response, Url, header::HeaderMap, header::HeaderName, header::HeaderValue,
};
use reqwest_middleware::{
    ClientBuilder, ClientWithMiddleware, Error, Middleware, Next, Result as MiddlewareResult,
};
use serde::de::DeserializeOwned;
use sha2::Sha256;
use task_local_extensions::Extensions;
use time::OffsetDateTime;
use tokio::sync::Mutex;
use types::{PongResponse, V3Error};

type HmacSha256 = Hmac<Sha256>;

/// Global singleton instance of CamClient, lazily initialized on first use
static CAM_CLIENT: Lazy<Arc<Mutex<CamClient>>> =
    Lazy::new(|| Arc::new(Mutex::new(CamClient::new())));

/// Get a reference to the global CamClient instance
#[tracing::instrument()]
pub async fn get_client() -> CamClient {
    CAM_CLIENT.lock().await.clone()
}

#[derive(Clone)]
pub struct CamClient {
    pub base_url: Url,
    pub client: ClientWithMiddleware,
}

impl CamClient {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        let base_url = env::var("CAM_BASE_URL").expect("CAM_BASE_URL is not set");
        let api_path = env::var("CAM_API_PATH").expect("CAM_API_PATH is not set");
        Self {
            base_url: Url::parse(&format!("{}{}/", base_url, api_path)).unwrap(),
            client: ClientBuilder::new(Client::new())
                .with(SigningMiddleware::new())
                .with(StatusCheckMiddleware::new())
                .build(),
        }
    }

    async fn parse_response<T: DeserializeOwned>(
        &self,
        resp: Response,
        method: &str,
        path: &str,
    ) -> Result<T> {
        assert!(resp.status().is_success());
        let text = resp.text().await?;
        serde_json::from_str::<T>(&text)
            .map_err(|e| anyhow!("Failed to parse response from {method} {path}: {e}"))
    }

    #[allow(dead_code)]
    pub async fn ping(&self) -> Result<PongResponse> {
        let path = "httpmisc/ping";
        let url = self.base_url.join(path).unwrap();
        let res = self.client.get(url).send().await.unwrap();
        self.parse_response(res, "GET", path).await
    }
}

impl Default for CamClient {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SigningMiddleware {
    api_path: String,
    api_key: String,
    api_secret: String,
}

impl SigningMiddleware {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        let api_path = env::var("CAM_API_PATH").expect("CAM_API_PATH is not set");
        let api_key = env::var("CAM_API_KEY").expect("CAM_API_KEY is not set");
        let api_secret = env::var("CAM_API_SECRET").expect("CAM_API_SECRET is not set");
        Self {
            api_path,
            api_key,
            api_secret,
        }
    }
}

impl Default for SigningMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Middleware for SigningMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> MiddlewareResult<Response> {
        let mut headers = HeaderMap::new();
        let timestamp = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000).to_string();

        let verb = req.method().to_string();
        let path = format!(
            "{}{}",
            req.url().path().replace(&self.api_path, ""),
            req.url()
                .query()
                .map_or("".to_owned(), |query| format!("?{}", query)),
        );
        let data = req.body().map_or("".to_owned(), |body| {
            String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap()
        });
        let req_msg = format!("{}{}{}{}", verb, path, timestamp, data);

        let decoded_secret = BASE64_STANDARD.decode(self.api_secret.clone()).unwrap();
        let mut mac = HmacSha256::new_from_slice(&decoded_secret).unwrap();
        mac.update(req_msg.as_bytes());
        let signature = BASE64_STANDARD.encode(mac.finalize().into_bytes().as_slice());

        headers.insert(
            HeaderName::from_static("api-timestamp"),
            HeaderValue::from_str(&timestamp).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("api-key"),
            HeaderValue::from_str(&self.api_key).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("api-signature"),
            HeaderValue::from_str(&signature).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        );

        req.headers_mut().extend(headers);
        let res = next.run(req, extensions).await?;
        Ok(res)
    }
}

pub struct StatusCheckMiddleware {}

impl StatusCheckMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StatusCheckMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Middleware for StatusCheckMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> MiddlewareResult<Response> {
        let path = req.url().path().to_string();
        let method = req.method().to_string();

        let resp = next.run(req, extensions).await?;
        let status = resp.status();
        if !status.is_success() {
            let query = resp.url().query().map(String::from);
            let error_title = format!(
                "{} {}{} status={}",
                method,
                path,
                query.map(|q| format!("?{}", q)).unwrap_or_default(),
                status
            );
            let error_text = resp.text().await?;

            if let Ok(e) = serde_json::from_str::<V3Error>(&error_text) {
                if e.code == "tick-not-in-redis" {
                    let re = Regex::new(r"(\S+)\s").unwrap();
                    if let Some(captures) = re.captures(&e.message) {
                        let token = captures[1].to_string();
                        let error = CamError::TokenPriceNotFound(token);
                        tracing::error!("{}", error);
                        return Err(Error::Middleware(anyhow!(error)));
                    }
                }
            }

            let error = CamError::RequestFailed(error_title);
            tracing::error!("{}", error);
            return Err(Error::Middleware(anyhow!(error)));
        }
        Ok(resp)
    }
}

/// Custom error types for CAM client
#[derive(Debug, thiserror::Error)]
pub enum CamError {
    #[error("Token not found: {0}")]
    TokenPriceNotFound(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),
}
