use serde::{Serialize,Deserialize};
use crate::{JsonRequest, Usage};
use async_trait::async_trait;


///text edit request as defined by https://platform.openai.com/docs/api-reference/edits
///
/// # Usage example
///```
///    use openai_req::edit::EditRequest;
///    use openai_req::JsonRequest;
///
///    let instruction = "correct spelling";
///    let text = "quick blck fox jupms over lazy dog";
///    let request = EditRequest::new_text(instruction).set_input(text);
///    let response = request.run(&client).await?;
/// ```
///
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EditRequest {
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<String>,
    instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[async_trait(?Send)]
impl JsonRequest<EditResponse> for EditRequest {
    const ENDPOINT: &'static str = "/edits";
}

impl EditRequest {

    pub fn new_text(instruction: &str) -> Self {
        Self {
            model: "text-davinci-edit-001".to_string(),
            input: None,
            instruction: instruction.to_string(),
            n: None,
            temperature: None,
            top_p: None,
        }
    }

    pub fn new_code(instruction: &str) -> Self {
        Self {
            model: "code-davinci-edit-001".to_string(),
            input: None,
            instruction: instruction.to_string(),
            n: None,
            temperature: None,
            top_p: None,
        }
    }

    pub fn with_model(model: &str, instruction: &str) -> Self {
        Self {
            model: model.to_string(),
            input: None,
            instruction: instruction.to_string(),
            n: None,
            temperature: None,
            top_p: None,
        }
    }

    pub fn set_input(mut self, input: &str) -> Self {
        self.input = Some(input.to_string());
        self
    }

    pub fn set_n(mut self, n: u16) -> Self {
        self.n = Some(n);
        self
    }

    pub fn set_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn set_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EditChoice {
    pub text: String,
    pub index: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EditResponse {
    pub object: String,
    pub created: i64,
    pub choices: Vec<EditChoice>,
    pub usage: Usage,
}

