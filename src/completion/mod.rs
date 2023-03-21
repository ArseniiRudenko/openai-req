use async_trait::async_trait;
use crate::{Input, JsonRequest, Usage};
use std::collections::HashMap;
use serde::{Serialize,Deserialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CompletionRequest {
    model: String,
    prompt: Input,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_of: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<HashMap<String, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}


#[async_trait(?Send)]
impl JsonRequest<CompletionSuccess> for CompletionRequest {
    const ENDPOINT: &'static str = "/completions";
}

impl CompletionRequest {
    pub fn new(prompt: Input) -> CompletionRequest {
        CompletionRequest {
            model: "text-davinci-003".to_string(),
            prompt,
            suffix: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }
    pub fn with_model(model: &str, prompt: Input) -> CompletionRequest {
        CompletionRequest {
            model: model.to_string(),
            prompt,
            suffix: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }

    pub fn set_suffix(&mut self, suffix: &str) -> &mut Self {
        self.suffix = Some(suffix.to_string());
        self
    }

    pub fn set_max_tokens(&mut self, max_tokens: u32) -> &mut Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn set_temperature(&mut self, temperature: f32) -> &mut Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn set_top_p(&mut self, top_p: f32) -> &mut Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn set_n(&mut self, n: u16) -> &mut Self {
        self.n = Some(n);
        self
    }

    pub fn set_stream(&mut self, stream: bool) -> &mut Self {
        self.stream = Some(stream);
        self
    }

    pub fn set_logprobs(&mut self, logprobs: u32) -> &mut Self {
        self.logprobs = Some(logprobs);
        self
    }

    pub fn set_echo(&mut self, echo: bool) -> &mut Self {
        self.echo = Some(echo);
        self
    }

    pub fn set_stop(mut self, stop: impl Into<Vec<String>>) -> Self {
        self.stop = Some(stop.into());
        self
    }

    pub fn set_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn set_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn set_best_of(mut self, best_of: u16) -> Self {
        self.best_of = Some(best_of);
        self
    }

    pub fn set_logit_bias(mut self, logit_bias: HashMap<String, f32>) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn set_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub  struct CompletionChoice {
    pub text: String,
    pub index: i64,
    pub logprobs: Option<u32>,
    pub finish_reason: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub  struct CompletionSuccess {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}
