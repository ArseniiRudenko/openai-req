
use crate::structs::Input;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingRequest{
    pub model:String,
    pub input:Input,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>
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

impl EmbeddingRequest {
    pub fn new(model: String, input: Input) -> Self {
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