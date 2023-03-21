use std::fmt::{Debug, Display, Formatter};
use serde::{Serialize, Deserialize};
use with_id::WithRefId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage{
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64
}

#[derive(Debug)]
pub struct Error{
    pub(crate) response:ErrorResponse,
    pub(crate) inner:reqwest::Error
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.response)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ErrorResponse{
    ApiError(ApiError),
    OtherError(String)
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorResponse::ApiError(a) => write!(f,"{}",a),
            ErrorResponse::OtherError(s) => write!(f,"{}",s)
        }
    }
}


impl Display for ApiError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error.param {
            None => match &self.error.code {
                None => write!(f,"{}",self.error.message),
                Some(code) => write!(f,"{}, code:{}",self.error.message,code)
            }
            Some(param) => match &self.error.code {
                None => write!(f,"{}, param:{}",self.error.message,param),
                Some(code) => write!(f,"{}, param:{}, code: {}",self.error.message,param,code)
            }
        }
    }
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

#[derive(Clone, Debug, Deserialize,Serialize,WithRefId)]
pub struct ModelRequest {
    #[id]
    pub model_name: String
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
    pub permission: Vec<ModelPermission>,
    pub root: String,
    pub parent: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
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

#[derive(Clone, Debug, Deserialize,Serialize)]
pub struct ModelListResponse {
    pub object: String,
    pub data: Vec<Model>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Input {
    String(String),
    StringArray(Vec<String>)
}

impl From<String> for Input{
    fn from(value:String) -> Self {
        Input::String(value)
    }
}

impl From<&str> for Input{
    fn from(value:&str) -> Self {
        Input::String(value.to_string())
    }
}

impl From<Vec<String>> for Input{
    fn from(value: Vec<String>) -> Self {
        Input::StringArray(value)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

