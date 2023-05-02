use super::gpt_chat::*;

pub enum ChatClientError {
    NetworkError,
    InvalidModel,
    InvalidRole,
    EmptyResponse,
    InvalidResponse,
    Unknown,
}

impl From<reqwest::Error> for ChatClientError {
    fn from(_: reqwest::Error) -> Self {
        Self::NetworkError
    }
}

pub struct ChatClient {
    api_key: String,
    reqwest_client: reqwest::Client,
    endpoint: String,
}

impl ChatClient {
    pub fn new<T>(api_key: T) -> ChatClient
    where
        T: Into<String>,
    {
        Self {
            api_key: api_key.into(),
            reqwest_client: reqwest::Client::new(),
            endpoint: "https://api.openai.com/v1/chat/completions".to_owned(),
        }
    }

    pub fn set_endpoint<T>(mut self, endpoint: T) -> ChatClient
    where
        T: Into<String>,
    {
        self.endpoint = endpoint.into();
        self
    }

    // TODO: Add proper error return/handling
    pub async fn get_completions(&self, request: Request) -> Result<Response, ChatClientError> {
        let response = self
            .reqwest_client
            .post(&self.endpoint)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        response
            .json::<Response>()
            .await
            .map_err(|_| ChatClientError::InvalidResponse)
    }
}
