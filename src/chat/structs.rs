use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use crate::structs::Usage;

#[derive(Clone, Debug,Serialize,Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message{
    pub role:Role,
    pub content:String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StopSeq{
    String(String),
    Vec(Vec<String>)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    model:String,
    messages:Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop:Option<StopSeq>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty:Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<HashMap<String,f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatChoice {
    pub index: u16,
    pub message: Message,
    pub finish_reason: String
}



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatSuccess {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<ChatChoice>,
    pub usage:Usage
}

impl ChatRequest {

    pub fn new(messages : Vec<Message>) -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }

    pub fn with_model_and_messages(model: &str, messages : Vec<Message>) -> Self {
        Self {
            model: model.to_string(),
            messages,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }

    pub fn add_message(mut self, message:Message) ->Self{
        self.messages.push(message);
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        if self.top_p.is_some() {
            self.top_p = None;
        }
        self.temperature = Some(temperature.clamp(0f64,2f64));
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        if self.temperature.is_some() {
            self.temperature = None;
        }
        self.top_p = Some(top_p.clamp(0f64,1f64));
        self
    }

    pub fn n(mut self, n: u16) -> Self {
        self.n = Some(n);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn stop(mut self, stop: StopSeq) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self{
        self.presence_penalty= Some(presence_penalty.clamp(-2f64,2f64));
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty.clamp(-2f64,2f64));
        self
    }

    pub fn logit_bias(mut self, logit_bias: HashMap<String, f32>) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

}
