use std::env;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResponse {
    pub traces: Vec<TraceSearchResult>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TraceSearchResult {
    #[serde(rename = "traceID")]
    pub trace_id: String,
    #[serde(default)]
    pub root_service_name: String,
    #[serde(default)]
    pub root_trace_name: String,
    #[serde(rename = "startTimeUnixNano")]
    pub start_time_unix_nano: String,
    #[serde(rename = "durationMs", default)]
    pub duration_ms: Option<u64>,
    #[serde(rename = "durationNanos", default)]
    pub duration_nanos: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TagsResponse {
    pub scopes: Vec<TagScope>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TagScope {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagValuesResponse {
    pub tag_values: Vec<TagValue>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TagValue {
    #[serde(rename = "type")]
    pub value_type: String,
    pub value: String,
}

pub struct TempoClient {
    client: Client,
    base_url: String,
    user: Option<String>,
    password: Option<String>,
}

impl TempoClient {
    pub fn new() -> Result<Self> {
        let base_url = env::var("TEMPO_URL").context("TEMPO_URL environment variable not set")?;
        let user = env::var("TEMPO_USER").ok();
        let password = env::var("TEMPO_PASSWORD").ok();

        Ok(Self {
            client: Client::new(),
            base_url,
            user,
            password,
        })
    }

    fn request(&self, path: &str) -> reqwest::blocking::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url);
        if let (Some(user), Some(pass)) = (&self.user, &self.password) {
            req = req.basic_auth(user, Some(pass));
        }
        req
    }

    pub fn get_trace(&self, trace_id: &str, start: Option<&str>, end: Option<&str>) -> Result<serde_json::Value> {
        let mut req = self.request(&format!("/api/traces/{}", trace_id));
        if let Some(s) = start {
            req = req.query(&[("start", s)]);
        }
        if let Some(e) = end {
            req = req.query(&[("end", e)]);
        }

        let resp = req
            .send()
            .context("Failed to send request")?
            .error_for_status()
            .context("Tempo returned an error")?
            .json()
            .context("Failed to parse response")?;

        Ok(resp)
    }

    pub fn search(
        &self,
        query: &str,
        start: Option<&str>,
        end: Option<&str>,
        limit: Option<u32>,
    ) -> Result<SearchResponse> {
        let mut req = self.request("/api/search").query(&[("q", query)]);
        if let Some(s) = start {
            req = req.query(&[("start", s)]);
        }
        if let Some(e) = end {
            req = req.query(&[("end", e)]);
        }
        if let Some(l) = limit {
            req = req.query(&[("limit", &l.to_string())]);
        }

        let resp = req
            .send()
            .context("Failed to send request")?
            .error_for_status()
            .context("Tempo returned an error")?
            .json()
            .context("Failed to parse response")?;

        Ok(resp)
    }

    pub fn tags(&self, start: Option<&str>, end: Option<&str>) -> Result<TagsResponse> {
        let mut req = self.request("/api/v2/search/tags");
        if let Some(s) = start {
            req = req.query(&[("start", s)]);
        }
        if let Some(e) = end {
            req = req.query(&[("end", e)]);
        }

        let resp = req
            .send()
            .context("Failed to send request")?
            .error_for_status()
            .context("Tempo returned an error")?
            .json()
            .context("Failed to parse response")?;

        Ok(resp)
    }

    pub fn tag_values(&self, tag: &str, start: Option<&str>, end: Option<&str>) -> Result<TagValuesResponse> {
        let mut req = self.request(&format!("/api/v2/search/tag/{}/values", tag));
        if let Some(s) = start {
            req = req.query(&[("start", s)]);
        }
        if let Some(e) = end {
            req = req.query(&[("end", e)]);
        }

        let resp = req
            .send()
            .context("Failed to send request")?
            .error_for_status()
            .context("Tempo returned an error")?
            .json()
            .context("Failed to parse response")?;

        Ok(resp)
    }
}
