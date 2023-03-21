pub mod structs;

use structs::{ChatRequest, ChatSuccess};
use async_trait::async_trait;
use crate::JsonRequest;

#[async_trait(?Send)]
impl JsonRequest<ChatSuccess> for ChatRequest {
    const ENDPOINT: &'static str = "/chat/completions";
}
