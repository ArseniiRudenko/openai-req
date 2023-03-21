pub mod structs;

use structs::{CompletionRequest, CompletionSuccess};
use async_trait::async_trait;
use crate::JsonRequest;

#[async_trait(?Send)]
impl JsonRequest<CompletionSuccess> for CompletionRequest {
    const ENDPOINT: &'static str = "/completions";
}