pub mod structs;

use super::{OpenAiClient, JsonRequestClient};
use structs::{CompletionRequest, CompletionSuccess};
use async_trait::async_trait;

#[async_trait(?Send)]
impl JsonRequestClient<CompletionRequest, CompletionSuccess> for OpenAiClient {
    const ENDPOINT: &'static str = "/completions";
}