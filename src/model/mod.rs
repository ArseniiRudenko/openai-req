use std::fmt::{Display, Formatter};
use derive_more::Constructor;
use reqwest::RequestBuilder;
use crate::{ByUrlRequest, DeleteResponse, GetRequest, OpenAiClient};
use with_id::WithRefId;
use serde::{Serialize,Deserialize};
use crate::fine_tunes::{FineTune, FineTuneListEntry};

/// allows to get info about single model from its name.
/// More details at https://platform.openai.com/docs/api-reference/models/retrieve
/// # Usage example
/// ```
/// use openai_req::ByUrlRequest;
/// use openai_req::model::ModelRequest;
///
/// let model_request = ModelRequest::new("model_name".to_string());
/// let model_info = model_request.run(&lclient).await?;
/// ```
#[derive(Clone, Debug, Deserialize,Serialize,WithRefId,Constructor)]
pub struct ModelRequest {
    #[id]
    model_name: String
}


impl TryFrom<FineTuneListEntry> for ModelRequest {
    type Error = ConversionError;

    fn try_from(value: FineTuneListEntry) -> Result<Self, Self::Error> {
        Ok(ModelRequest {
            model_name: value
                .fine_tuned_model
                .ok_or(ConversionError{
                    message:"can only convert finished fine tune, that has fine tune model set"
                })?
        })
    }
}

impl TryFrom<FineTune> for ModelRequest {

    type Error = ConversionError;

    fn try_from(value: FineTune) -> Result<Self, Self::Error> {
        Ok(ModelRequest {
            model_name: value
                .fine_tuned_model
                .ok_or(ConversionError{
                    message:"can only convert finished fine tune, that has fine tune model set"
                })?
        })
    }
}


impl ByUrlRequest<Model> for ModelRequest{
    const ENDPOINT: &'static str = "/models/";
    const SUFFIX: &'static str = "";
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

/// List of all available models.
/// More details at https://platform.openai.com/docs/api-reference/models/list
/// # Usage example
/// ```
/// use openai_req::GetRequest;
/// use openai_req::model::ModelListResponse;
///
/// let response = ModelListResponse::get(&client).await?;
/// ```
#[derive(Clone, Debug, Deserialize,Serialize)]
pub struct ModelListResponse {
    pub object: String,
    pub data: Vec<Model>
}

impl GetRequest for ModelListResponse {
    const ENDPOINT: &'static str = "/models";
}

/// allows deleting owned models. Part of the fine-tune API.
/// More details at https://platform.openai.com/docs/api-reference/fine-tunes/delete-model
/// # Usage example
/// ```
/// use openai_req::ByUrlRequest;
/// use openai_req::model::ModelDeleteRequest;
///
/// let req = ModelDeleteRequest::new("model_name".to_string());
/// let res = req.run(&client).await?;
/// ```
///
#[derive(Serialize, Deserialize, Debug, Clone,WithRefId,Constructor)]
pub struct ModelDeleteRequest {
    #[id]
    model_name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversionError{
    message:&'static str
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl std::error::Error for ConversionError {}


impl TryFrom<FineTuneListEntry> for ModelDeleteRequest {
    type Error = ConversionError;

    fn try_from(value: FineTuneListEntry) -> Result<Self, Self::Error> {
        Ok(ModelDeleteRequest {
            model_name: value
                .fine_tuned_model
                .ok_or(ConversionError{
                    message:"can only convert finished fine tune, that has fine tune model set"
                })?
        })
    }
}

impl TryFrom<FineTune> for ModelDeleteRequest {

    type Error = ConversionError;

    fn try_from(value: FineTune) -> Result<Self, Self::Error> {
        Ok(ModelDeleteRequest {
            model_name: value
                .fine_tuned_model
                .ok_or(ConversionError{
                    message:"can only convert finished fine tune, that has fine tune model set"
                })?
        })
    }
}


impl ByUrlRequest<DeleteResponse> for ModelDeleteRequest {
    const ENDPOINT: &'static str = "/models/";
    const SUFFIX: &'static str = "";

    fn builder(client:&OpenAiClient, final_url: String) -> RequestBuilder {
        client.client.delete(final_url)
    }
}
