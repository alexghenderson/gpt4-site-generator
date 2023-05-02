mod cache_client;
mod chat_client;

use std::net::SocketAddr;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;

use chat_client::{ChatClient, Message, Model, Request, Role};

use crate::cache_client::{Cache, MemoryCache};
use crate::chat_client::ChatClientError;

#[derive(Clone)]
struct AppState {
    chat_client: Arc<ChatClient>,
    cache_client: Arc<Mutex<MemoryCache<(String, String), String>>>,
}

#[tokio::main]
async fn main() {
    let client = AppState {
        chat_client: Arc::new(ChatClient::new(
            std::env::var("OPENAPI_KEY").expect("OPENAPI_KEY environment variable required"),
        )),
        cache_client: Arc::new(Mutex::new(MemoryCache::new())),
    };

    let app = Router::new()
        .route("/:category/:topic", get(root))
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Starting server");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start axum");
}

async fn root(
    Path((category, topic)): Path<(String, String)>,
    State(AppState {
        chat_client,
        cache_client,
    }): State<AppState>,
) -> Response {
    if let Some(response) = cache_client
        .lock()
        .unwrap()
        .read_key(&(category.clone(), topic.clone()))
    {
        return axum::response::Html(response.clone()).into_response();
    }
    let prompt = "You are a site content generator. You generate a full, valid HTML page for a website for a given catagory and topic. The contents should be similar to an encyclopedia in nature, and include related links in the form o f`/:category/:topic`.";
    let response = chat_client
        .get_completions(
            Request::new(Model::GPT4)
                .add_message(Message::new(Role::Assistant, prompt))
                .add_message(Message::new(
                    Role::User,
                    format!(
                        "Generate a site. The category is `{}` and the topic is `{}`",
                        category, topic
                    ),
                )),
        )
        .await
        .map(|response| {
            response
                .first_choice()
                .map(|choice| choice.get_message().get_content().clone())
        });

    use axum::http::StatusCode;
    match response {
        Ok(Some(response)) => {
            cache_client
                .lock()
                .unwrap()
                .write_key((category.clone(), topic.clone()), response.clone())
                .expect("Failed to write cache");
            axum::response::Html(response).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Not Found").into_response(),
        Err(ChatClientError::NetworkError) => {
            (StatusCode::REQUEST_TIMEOUT, "Request Timeout").into_response()
        }
        Err(ChatClientError::EmptyResponse) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response(),
    }
}
