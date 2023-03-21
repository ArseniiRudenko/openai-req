
use crate::{Input, JsonRequest};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingRequest{
    pub model:String,
    pub input:Input,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>
}

impl JsonRequest<EmbeddingResponse> for EmbeddingRequest{
    const ENDPOINT: &'static str = "/embeddings";
}

impl EmbeddingRequest {
    pub fn new(input: Input) -> Self {
        EmbeddingRequest {
            model: "text-embedding-ada-002".to_string(),
            input,
            user: None,
        }
    }


    pub fn with_model(model: String, input: Input) -> Self {
        EmbeddingRequest {
            model,
            input,
            user: None,
        }
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Struct {
    pub object: String,
    pub embedding: Vec<f64>,
    pub index: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<Struct>,
    pub model: String,
    pub usage: Usage,
}
