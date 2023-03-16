use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage{
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApiResponse<T>{
    Ok(T),
    Error(ErrorResponse)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ErrorResponse{
    ApiError(ApiError),
    OtherError(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    pub error: ApiErrorDetails
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename(serialize = "error"))]
#[serde(rename(deserialize = "error"))]
pub struct ApiErrorDetails {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub param: Option<String>,
    pub code: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilesResponse {
    pub data: Vec<FileInfo>,
    pub object: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
    pub permission: Vec<ModelPermission>,
    pub root: String,
    pub parent: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelPermission {
    pub  id: String,
    pub object: String,
    pub created: i64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<Model>
}

