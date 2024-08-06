use crate::constants;
use crate::http_client::HttpClient;
use crate::jpush_resp_err::JPushResponseErr;
use serde::{Deserialize, Serialize};

pub struct JPushClient {}
impl JPushClient {
    pub fn new() -> Self {
        JPushClient {}
    }
    pub async fn push(&self, url: &str, body: &str) -> Result<String, JPushResponseErr> {
        // let client = reqwest::Client::new();
        // let resp = client
        //     .post(url)
        //     .header("Content-Type", "application/json")
        //     .body(body)
        //     .send().await;
        Ok(String::from(""))
    }
}

// 请求参数信息
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushRequest {
    pub base: BaseRequest,
    pub page: u32,
    pub limit: u32,
}

impl PushRequest {
    pub fn new(page: u32, limit: u32) -> Self {
        PushRequest {
            base: BaseRequest {
                method: "GET".to_string(),
                url: "https://api.example.com/todos".to_string(),
                headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            },
            page,
            limit,
        }
    }
}