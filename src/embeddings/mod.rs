use crate::embeddings::structs::{EmbeddingRequest, EmbeddingResponse};
use crate::{JsonRequest, OpenAiClient};

pub mod structs;

impl JsonRequest<EmbeddingResponse> for EmbeddingRequest{
    const ENDPOINT: &'static str = "/embeddings";
}
