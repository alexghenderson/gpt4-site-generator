use super::models::*;
use anyhow::Error;
use async_trait::async_trait;
use reqwest;

#[async_trait]
pub trait OpenAIClient {
    async fn get_chat_completions(
        &mut self,
        request: ChatCompletionsRequest,
    ) -> Result<ChatCompletionsResponse, Error>;
}

pub struct Client {
    reqwest_client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String, reqwest_client: reqwest::Client) -> Self {
        Self {
            reqwest_client,
            base_url: "https://api.openai.com/v1".to_owned(),
            api_key,
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn with_client(mut self, reqwest_client: reqwest::Client) -> Self {
        self.reqwest_client = reqwest_client;
        self
    }
}

#[async_trait]
impl OpenAIClient for Client {
    async fn get_chat_completions(
        &mut self,
        request: ChatCompletionsRequest,
    ) -> Result<ChatCompletionsResponse, anyhow::Error> {
        self.reqwest_client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json; charset=utf8;")
            .json(&request)
            .send()
            .await?
            .text()
            .await
            .map(|text| {
                println!("{}", text);
                serde_json::from_str::<ChatCompletionsResponse>(text.as_str())
            })?
            // .json::<ChatCompletionsResponse>()
            .map_err(|err| err.into())
    }
}
