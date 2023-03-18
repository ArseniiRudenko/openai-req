pub mod structs;

use structs::{ChatRequest, ChatSuccess};
use super::{OpenAiClient, PostClient};
use async_trait::async_trait;

#[async_trait(?Send)]
impl PostClient<ChatRequest, ChatSuccess> for OpenAiClient {
    const ENDPOINT: &'static str = "/chat/completions";
}
