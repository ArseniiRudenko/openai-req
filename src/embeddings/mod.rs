use crate::embeddings::structs::{EmbeddingRequest, EmbeddingResponse};
use crate::{OpenAiClient, JsonRequestClient};

pub mod structs;

impl JsonRequestClient<EmbeddingRequest,EmbeddingResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/embeddings";
}
