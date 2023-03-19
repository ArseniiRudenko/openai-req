use crate::files::structs::FileInfo;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuneCreateRequest {
    training_file: String,
    validation_file: Option<String>,
    model: Option<String>,
    n_epochs: Option<i32>,
    batch_size: Option<i32>,
    learning_rate_multiplier: Option<f64>,
    prompt_loss_weight: Option<f64>,
    compute_classification_metrics: Option<bool>,
    classification_n_classes: Option<i32>,
    classification_positive_class: Option<String>,
    classification_betas: Option<Vec<f64>>,
    suffix: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hyperparams {
    pub batch_size: i64,
    pub learning_rate_multiplier: f64,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTune {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: i64,
    pub events: Vec<FineTuneEvent>,
    pub fine_tuned_model: Option<String>,
    pub hyperparams: Hyperparams,
    pub organization_id: String,
    pub result_files: Vec<FileInfo>,
    pub status: String,
    pub validation_files: Vec<FileInfo>,
    pub training_files: Vec<FileInfo>,
    pub updated_at: i64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuneListEntry {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: i64,
    pub fine_tuned_model: Option<String>,
    pub hyperparams: Hyperparams,
    pub organization_id: String,
    pub result_files: Vec<FileInfo>,
    pub status: String,
    pub validation_files: Vec<FileInfo>,
    pub training_files: Vec<FileInfo>,
    pub updated_at: i64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTunesResponse{
    pub object: String,
    pub data: Vec<FineTuneListEntry>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuneEventsResponse{
    pub object: String,
    pub data: Vec<FineTuneEvent>
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