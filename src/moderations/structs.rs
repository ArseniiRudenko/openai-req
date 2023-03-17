use crate::structs::Input;
use serde::*;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub  struct Struct {
    pub categories: Categories,
    pub category_scores: CategoryScores,
    pub flagged: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<Struct>,
}

#[derive(Serialize, Deserialize)]
pub enum ModerationModel{
    #[serde(rename = "text-moderation-stable")]
    TextModerationStable,
    #[serde(rename = "text-moderation-latest")]
    TextModerationLatest
}

#[derive(Serialize, Deserialize)]
pub struct ModerationRequest{
    pub input:Input,
    pub model:ModerationModel
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