use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage{
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiResponse<T>{
    Ok(T),
    Error(ErrorResponse)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ErrorResponse{
    ApiError(ApiError),
    OtherError(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub error: ApiErrorDetails
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "error"))]
#[serde(rename(deserialize = "error"))]
pub struct ApiErrorDetails {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub param: Option<String>,
    pub code: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilesResponse {
    pub data: Vec<FileInfo>,
    pub object: String,
}