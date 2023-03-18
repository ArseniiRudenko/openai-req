use crate::embeddings::structs::{EmbeddingRequest, EmbeddingResponse};
use crate::{OpenAiClient, PostClient};

pub mod structs;

impl PostClient<EmbeddingRequest,EmbeddingResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/embeddings";
}
