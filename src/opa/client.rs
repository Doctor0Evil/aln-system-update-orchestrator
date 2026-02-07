use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone)]
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decision {
    pub allowed: bool,
    pub details: Value,
}

impl Client {
    pub fn new<S: Into<String>>(base_url: S) -> Self {
        Self {
            base_url: base_url.into(),
            http: reqwest::Client::new(),
        }
    }

    pub async fn evaluate(&self, path: &str, input: Value) -> Result<Decision> {
        let url = format!("{}/v1/data/{}", self.base_url.trim_end_matches('/'), path);
        let resp = self
            .http
            .post(url)
            .json(&serde_json::json!({ "input": input }))
            .send()
            .await?;

        let json: Value = resp.json().await?;
        let allowed = json.pointer("/result").unwrap_or(&Value::Bool(true)) == &Value::Bool(true);
        Ok(Decision {
            allowed,
            details: json,
        })
    }
}
