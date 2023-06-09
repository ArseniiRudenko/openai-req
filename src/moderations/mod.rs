
use crate::{Input, JsonRequest};
use serde::*;

/// Obtains moderation info for provided text.
/// More details at https://platform.openai.com/docs/api-reference/moderations
/// # Usage example
/// ```
/// use openai_req::JsonRequest;
/// use openai_req::moderations::ModerationRequest;
/// 
/// let req = ModerationRequest::new("I want to kill everyone".into());
/// let res = req.run(&client).await?;
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModerationRequest{
    pub input:Input,
    pub model:ModerationModel
}

impl JsonRequest<ModerationResponse> for ModerationRequest{
    const ENDPOINT: &'static str = "/moderations";
}

impl ModerationRequest{
    pub fn new(input:Input) -> Self {
        ModerationRequest{
            input,
            model:ModerationModel::TextModerationStable
        }
    }

    pub fn with_model(model:ModerationModel,input:Input) -> Self {
        ModerationRequest{
            input,
            model
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategoryScores {
    pub hate: f64,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,
    #[serde(rename = "self-harm")]
    pub self_harm: f64,
    pub sexual: f64,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,
    pub violence: f64,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Categories {
    pub hate: bool,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    pub sexual: bool,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    pub violence: bool,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub  struct Struct {
    pub categories: Categories,
    pub category_scores: CategoryScores,
    pub flagged: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<Struct>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ModerationModel{
    #[serde(rename = "text-moderation-stable")]
    TextModerationStable,
    #[serde(rename = "text-moderation-latest")]
    TextModerationLatest
}