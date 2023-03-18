pub mod structs;

use super::{OpenAiClient, PostClient};
use structs::{CompletionRequest, CompletionSuccess};
use async_trait::async_trait;

#[async_trait(?Send)]
impl PostClient<CompletionRequest, CompletionSuccess> for OpenAiClient {
    const ENDPOINT: &'static str = "/completions";
}