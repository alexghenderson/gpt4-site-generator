use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "null")]
    Null,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionsRequest {
    model: Model,
    messages: Vec<Message>,
}

impl ChatCompletionsRequest {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            messages: vec![],
        }
    }

    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Model {
    #[serde(rename = "gpt-4")]
    GPT4,
    #[serde(rename = "gpt-3.5-turbo")]
    GPT3_5Turbo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionsResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: TokenUsage,
    choices: Vec<Choice>,
}

impl ChatCompletionsResponse {
    pub fn first_choice(&self) -> Option<&Choice> {
        self.choices.first()
    }

    pub fn get_choices(&self) -> &Vec<Choice> {
        &self.choices
    }

    pub fn get_model(&self) -> &String {
        &self.model
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_created(&self) -> u64 {
        self.created
    }

    pub fn get_usage(&self) -> TokenUsage {
        self.usage
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct TokenUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl TokenUsage {
    pub fn prompt(&self) -> u32 {
        self.prompt_tokens
    }

    pub fn completion(&self) -> u32 {
        self.completion_tokens
    }

    pub fn total(&self) -> u32 {
        self.total_tokens
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ValidationError {
    Role(String),
    FinishReason(String),
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    message: Message,
    finish_reason: FinishReason,
    index: u32,
}

impl Choice {
    pub fn get_message(&self) -> &Message {
        &self.message
    }

    pub fn get_finish_reason(&self) -> FinishReason {
        self.finish_reason
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub fn new<T>(role: Role, content: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_role(&self) -> Role {
        self.role
    }
}

#[derive(Serialize, Deserialize, Debug, thiserror::Error)]
pub struct NoChoicesError {}

impl std::fmt::Display for NoChoicesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("No choices returned for prompt")
    }
}
