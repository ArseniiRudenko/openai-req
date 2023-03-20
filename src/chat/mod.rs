pub mod structs;

use structs::{ChatRequest, ChatSuccess};
use super::{OpenAiClient, JsonRequestClient};
use async_trait::async_trait;

#[async_trait(?Send)]
impl JsonRequestClient<ChatRequest, ChatSuccess> for OpenAiClient {
    const ENDPOINT: &'static str = "/chat/completions";
}
