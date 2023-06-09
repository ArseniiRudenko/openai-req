use crate::{GetRequest, JsonRequest, ByUrlRequest, OpenAiClient};

use reqwest::RequestBuilder;

use derive_more::*;
use serde::{Serialize,Deserialize};
use with_id::WithRefId;


///create fine-tune request as in https://platform.openai.com/docs/api-reference/fine-tunes/create
/// # Usage example
///```
/// use openai_req::ByUrlRequest;
/// use openai_req::files::FileInfoRequest;
///
/// let info_request= FileInfoRequest::new("training_file_id".to_string());
/// let info = info_request.run(&client).await?;
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuneCreateRequest {
    training_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_epochs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    learning_rate_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_loss_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_classification_metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_n_classes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_positive_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_betas: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
}

impl JsonRequest<FineTune> for FineTuneCreateRequest{
    const ENDPOINT: &'static str = "/fine-tunes";
}

impl FineTuneCreateRequest {
    pub fn new(training_file: String) -> Self {
        FineTuneCreateRequest {
            training_file,
            validation_file: None,
            model: None,
            n_epochs: None,
            batch_size: None,
            learning_rate_multiplier: None,
            prompt_loss_weight: None,
            compute_classification_metrics: None,
            classification_n_classes: None,
            classification_positive_class: None,
            classification_betas: None,
            suffix: None,
        }
    }

    pub fn validation_file(mut self, validation_file: String) -> Self {
        self.validation_file = Some(validation_file);
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn n_epochs(mut self, n_epochs: i32) -> Self {
        self.n_epochs = Some(n_epochs);
        self
    }

    pub fn batch_size(mut self, batch_size: i32) -> Self {
        self.batch_size = Some(batch_size);
        self
    }

    pub fn learning_rate_multiplier(mut self, learning_rate_multiplier: f64) -> Self {
        self.learning_rate_multiplier = Some(learning_rate_multiplier);
        self
    }

    pub fn prompt_loss_weight(mut self, prompt_loss_weight: f64) -> Self {
        self.prompt_loss_weight = Some(prompt_loss_weight);
        self
    }

    pub fn compute_classification_metrics(mut self, compute_classification_metrics: bool) -> Self {
        self.compute_classification_metrics = Some(compute_classification_metrics);
        self
    }

    pub fn classification_n_classes(mut self, classification_n_classes: i32) -> Self {
        self.classification_n_classes = Some(classification_n_classes);
        self
    }

    pub fn classification_positive_class(mut self, classification_positive_class: String) -> Self {
        self.classification_positive_class = Some(classification_positive_class);
        self
    }

    pub fn classification_betas(mut self, classification_betas: Vec<f64>) -> Self {
        self.classification_betas = Some(classification_betas);
        self
    }

    pub fn suffix(mut self, suffix: String) -> Self {
        self.suffix = Some(suffix);
        self
    }
}

/// Cancel fine tune request.
/// Details at https://platform.openai.com/docs/api-reference/fine-tunes/cancel
/// # Usage example
/// ```
/// use openai_req::ByUrlRequest;
/// use openai_req::fine_tunes::FineTuneCancelRequest;
///
/// let request = FineTuneCancelRequest::new("fine_tune_id".to_string());
/// let result = request.run(&client).await?;
/// ```
#[derive(Serialize, Deserialize, Debug, Clone,WithRefId, Constructor)]
pub struct FineTuneCancelRequest{
    id:String
}

impl From<FineTuneListEntry> for FineTuneCancelRequest{
    fn from(value: FineTuneListEntry) -> Self {
        FineTuneCancelRequest{
            id:value.id
        }
    }
}

impl From<FineTune> for FineTuneCancelRequest{
    fn from(value: FineTune) -> Self {
        FineTuneCancelRequest{
            id: value.id
        }
    }
}

impl ByUrlRequest<FineTune> for FineTuneCancelRequest {
    const ENDPOINT: &'static str = "/fine-tunes/";
    const SUFFIX: &'static str = "/cancel";

    fn builder(client:&OpenAiClient, final_url: String) -> RequestBuilder {
        client.client.post(final_url)
    }
}

///Get list of all available fine-tunes
/// Details at https://platform.openai.com/docs/api-reference/fine-tunes/list
/// # Usage example
///```
/// use openai_req::fine_tunes::FineTuneListResponse;
/// use openai_req::GetRequest;
///
/// let fine_tunes = FineTuneListResponse::get(&client).await?;
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuneListResponse {
    pub object: String,
    pub data: Vec<FineTuneListEntry>
}

impl GetRequest for FineTuneListResponse {
    const ENDPOINT: &'static str = "/fine-tunes";
}

#[derive(Serialize, Deserialize, Debug, Clone, WithRefId)]
pub struct FineTuneListEntry {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: i64,
    pub fine_tuned_model: Option<String>,
    pub hyperparams: Hyperparams,
    pub organization_id: String,
    pub result_files: Vec<FineTuneFileInfo>,
    pub status: String,
    pub validation_files: Vec<FineTuneFileInfo>,
    pub training_files: Vec<FineTuneFileInfo>,
    pub updated_at: i64,
}


///Get information about single fine tune.
/// Details at https://platform.openai.com/docs/api-reference/fine-tunes/retrieve
/// # Usage example
///```
/// use openai_req::ByUrlRequest;
/// use openai_req::fine_tunes::FineTuneGetRequest;
///
/// let req = FineTuneGetRequest::new("fine_tune_id".to_string());
/// let res = req.run(&client).await?;
/// ```
#[derive(Serialize, Deserialize, Debug, Clone,WithRefId, Constructor)]
pub struct FineTuneGetRequest{
    id: String
}

impl From<FineTuneListEntry> for FineTuneGetRequest{
    fn from(value: FineTuneListEntry) -> Self {
        FineTuneGetRequest{
            id: value.id
        }
    }
}

impl ByUrlRequest<FineTune> for FineTuneGetRequest {
    const ENDPOINT: &'static str = "/fine-tunes/";
    const SUFFIX: &'static str = "";
}

#[derive(Serialize, Deserialize, Debug, Clone, WithRefId)]
pub struct FineTune {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: i64,
    pub events: Vec<FineTuneEvent>,
    pub fine_tuned_model: Option<String>,
    pub hyperparams: Hyperparams,
    pub organization_id: String,
    pub result_files: Vec<FineTuneFileInfo>,
    pub status: String,
    pub validation_files: Vec<FineTuneFileInfo>,
    pub training_files: Vec<FineTuneFileInfo>,
    pub updated_at: i64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hyperparams {
    pub batch_size: Option<i64>,
    pub learning_rate_multiplier: Option<f64>,
    pub n_epochs: i64,
    pub prompt_loss_weight: f64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuneEvent {
    pub object: String,
    pub created_at: i64,
    pub level: String,
    pub message: String,
}

///Provides same fields as files::FileInfo(and can be converted `.into()` it)
///but also provides additional status information in context of fine-tune.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuneFileInfo{
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
    pub status: String,
    pub status_details: Option<String>
}

/// Get fine-tune events for certain fine-tune.
/// More details at https://platform.openai.com/docs/api-reference/fine-tunes/events
/// # Usage example
/// ```
/// use openai_req::ByUrlRequest;
/// use openai_req::fine_tunes::FineTuneEventsGetRequest;
///
/// let req = FineTuneEventsGetRequest::new("fine_tune_id".to_string());
/// let events = req.run(&client).await?;
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, WithRefId, Constructor)]
pub struct FineTuneEventsGetRequest{
    id: String
}

impl From<FineTuneListEntry> for FineTuneEventsGetRequest{
    fn from(value: FineTuneListEntry) -> Self {
        FineTuneEventsGetRequest{
            id:value.id
        }
    }
}

impl From<FineTune> for FineTuneEventsGetRequest{
    fn from(value: FineTune) -> Self {
        FineTuneEventsGetRequest{
            id: value.id
        }
    }
}

impl ByUrlRequest<FineTuneEventsResponse> for FineTuneEventsGetRequest{
    const ENDPOINT: &'static str = "/fine-tunes/";
    const SUFFIX: &'static str = "/events";
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuneEventsResponse{
    pub object: String,
    pub data: Vec<FineTuneEvent>
}





