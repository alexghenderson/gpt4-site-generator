#[macro_use]
extern crate rocket;

mod cache_client;
mod chat_client;
mod openai_client;

use anyhow::{anyhow, Error};
use rocket::futures::lock::Mutex;
use std::net::SocketAddr;
use std::sync::Arc;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};

use crate::cache_client::{Cache, MemoryCache};
use crate::openai_client::client::Client as OpenAIClient;

const SYSTEM_PROMPT: &'static str = "You are a site content generator. You generate a full, valid HTML page for a website for a given catagory and topic. The contents should be similar to an encyclopedia in nature, and include related links in the form o f`/:category/:topic`.";

struct AppState {
    cache_client: Mutex<MemoryCache<(String, String), String>>,
    openai_client: Mutex<OpenAIClient>,
}

#[launch]
async fn rocket() -> _ {
    let state = Arc::new(AppState {
        openai_client: Mutex::new(OpenAIClient::new(
            std::env::var("OPENAI_KEY").expect("OPENAI_KEY environment variable is required"),
            reqwest::Client::new(),
        )),
        cache_client: Mutex::new(MemoryCache::new()),
    });

    rocket::build().mount("/", routes![root]).manage(state)
}

struct HtmlResponse(String);

impl From<String> for HtmlResponse {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<'r> Responder<'r, 'r> for HtmlResponse {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'r> {
        Response::build_from(self.0.respond_to(req)?)
            .header(ContentType::HTML)
            .status(Status::Ok)
            .ok()
    }
}

struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl<'r> Responder<'r, 'r> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'r> {
        Response::build_from(self.0.to_string().respond_to(req)?)
            .status(Status::InternalServerError)
            .ok()
    }
}

type AppResponse = Result<HtmlResponse, AppError>;

#[get("/<category>/<topic>")]
async fn root(
    category: String,
    topic: String,
    state: &rocket::State<Arc<AppState>>,
) -> AppResponse {
    use openai_client::client::OpenAIClient;
    use openai_client::models::*;
    {
        if let Some(response) = state
            .cache_client
            .lock()
            .await
            .read_key(&(category.clone(), topic.clone()))
        {
            return Ok(response.clone().into());
        }
    }

    let result = state
        .clone()
        .openai_client
        .lock()
        .await
        .get_chat_completions(
            ChatCompletionsRequest::new(Model::GPT4)
                .add_message(Message::new(Role::Assistant, SYSTEM_PROMPT))
                .add_message(Message::new(
                    Role::User,
                    format!(
                        "Generate a site. The category is `{}` and the topic is `{}`",
                        category, topic
                    ),
                )),
        )
        .await
        .map_err(|e| {
            println!("{:?}", e);
            e
        })?;

    let response = result
        .first_choice()
        .ok_or(anyhow::Error::from(NoChoicesError {}))?
        .get_message()
        .get_content()
        .clone();

    state
        .cache_client
        .lock()
        .await
        .write_key((category.clone(), topic.clone()), response.clone())
        .expect("Failed to write cache");

    Ok(response.clone().into())
}
