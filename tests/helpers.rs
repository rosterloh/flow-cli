// tests/helpers.rs
use std::sync::{Arc, Mutex};

use anyhow::Result;
use reqwest::Method;
use serde_json::Value;

use flow_cli::client::HttpSend;

#[derive(Debug, Clone)]
pub struct MockCall {
    pub method: String,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub body: Option<Value>,
}

pub struct MockHttpClient {
    response: Value,
    calls: Arc<Mutex<Vec<MockCall>>>,
}

impl MockHttpClient {
    pub fn with_response(response: Value) -> Self {
        Self {
            response,
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn calls(&self) -> Vec<MockCall> {
        self.calls.lock().unwrap().clone()
    }
}

impl HttpSend for MockHttpClient {
    async fn send(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        _with_auth: bool,
    ) -> Result<Value> {
        self.calls.lock().unwrap().push(MockCall {
            method: method.to_string(),
            path: path.to_string(),
            query: query.to_vec(),
            body,
        });
        Ok(self.response.clone())
    }
}
