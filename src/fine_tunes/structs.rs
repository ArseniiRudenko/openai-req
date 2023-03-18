use crate::files::structs::FileInfo;
use serde::{Serialize,Deserialize};
use crate::structs::Model;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FineTuneRequest {
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
struct Hyperparams {
    pub batch_size: i64,
    pub learning_rate_multiplier: f64,
    pub n_epochs: i64,
    pub prompt_loss_weight: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FineTuneEvent {
    pub object: String,
    pub created_at: i64,
    pub level: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FineTune {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: i64,
    pub events: Vec<FineTuneEvent>,
    pub fine_tuned_model: Option<Model>,
    pub hyperparams: Hyperparams,
    pub organization_id: String,
    pub result_files: Vec<FileInfo>,
    pub status: String,
    pub validation_files: Vec<FileInfo>,
    pub training_files: Vec<FileInfo>,
    pub updated_at: i64,
}

impl FineTuneRequest {
    fn new(training_file: String) -> Self {
        FineTuneRequest {
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

    fn validation_file(mut self, validation_file: String) -> Self {
        self.validation_file = Some(validation_file);
        self
    }

    fn model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    fn n_epochs(mut self, n_epochs: i32) -> Self {
        self.n_epochs = Some(n_epochs);
        self
    }

    fn batch_size(mut self, batch_size: i32) -> Self {
        self.batch_size = Some(batch_size);
        self
    }

    fn learning_rate_multiplier(mut self, learning_rate_multiplier: f64) -> Self {
        self.learning_rate_multiplier = Some(learning_rate_multiplier);
        self
    }

    fn prompt_loss_weight(mut self, prompt_loss_weight: f64) -> Self {
        self.prompt_loss_weight = Some(prompt_loss_weight);
        self
    }

    fn compute_classification_metrics(mut self, compute_classification_metrics: bool) -> Self {
        self.compute_classification_metrics = Some(compute_classification_metrics);
        self
    }

    fn classification_n_classes(mut self, classification_n_classes: i32) -> Self {
        self.classification_n_classes = Some(classification_n_classes);
        self
    }

    fn classification_positive_class(mut self, classification_positive_class: String) -> Self {
        self.classification_positive_class = Some(classification_positive_class);
        self
    }

    fn classification_betas(mut self, classification_betas: Vec<f64>) -> Self {
        self.classification_betas = Some(classification_betas);
        self
    }

    fn suffix(mut self, suffix: String) -> Self {
        self.suffix = Some(suffix);
        self
    }
}