use std::fmt::{Display, Formatter};
use reqwest::RequestBuilder;
use crate::{ByUrlRequest, DeleteResponse, GetRequest, OpenAiClient};
use with_id::WithRefId;
use serde::{Serialize,Deserialize};
use crate::fine_tunes::structs::{FineTune, FineTuneListEntry};

/// allows to get info about single model from its name
#[derive(Clone, Debug, Deserialize,Serialize,WithRefId)]
pub struct ModelRequest {
    #[id]
    pub model_name: String
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

/// list of all available models, doesnt require request, has get method
#[derive(Clone, Debug, Deserialize,Serialize)]
pub struct ModelListResponse {
    pub object: String,
    pub data: Vec<Model>
}

impl GetRequest for ModelListResponse {
    const ENDPOINT: &'static str = "/models";
}

/// allows deleting owned models
#[derive(Serialize, Deserialize, Debug, Clone,WithRefId)]
pub struct ModelDeleteRequest {
    pub id: String
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
            id: value
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
            id: value
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
