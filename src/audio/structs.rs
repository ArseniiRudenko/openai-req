use std::io;
use std::path::{PathBuf};
use reqwest::multipart::{Form, Part};
use serde::{Serialize,Deserialize};
use crate::{AsyncTryFrom, file_to_part};
use crate::structs::Iso639_1;
use async_trait::async_trait;

#[derive(Clone, Debug,Serialize,Deserialize)]
pub enum ResponseFormat{

    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt
}

impl ToString for ResponseFormat {
    fn to_string(&self) -> String {
        match self {
            ResponseFormat::Json => "json".to_string(),
            ResponseFormat::Text => "text".to_string(),
            ResponseFormat::Srt => "srt".to_string(),
            ResponseFormat::VerboseJson => "verbose_json".to_string(),
            ResponseFormat::Vtt => "vtt".to_string()
        }
    }
}


#[derive(Clone, Debug)]
pub struct TranscriptionRequest{
    file: PathBuf,
    model: String,
    prompt:Option<String>,
    response_format: Option<ResponseFormat>,
    temperature: Option<f64>,
    language: Option<Iso639_1>
}


#[derive(Clone, Debug)]
pub struct TranslationRequest{
    file: PathBuf,
    model: String,
    prompt:Option<String>,
    response_format: Option<ResponseFormat>,
    temperature: Option<f64>
}



#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct  AudioResponse{
    pub text:String
}


impl TranscriptionRequest {
    pub fn new(file: PathBuf) -> Self {
        TranscriptionRequest {
            file,
            model: "whisper-1".to_string(),
            prompt: None,
            response_format: None,
            temperature: None,
            language: None
        }
    }

    pub fn with_model(file: PathBuf, model: String) -> Self {
        TranscriptionRequest {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None,
            language: None
        }
    }

    pub fn file(mut self, file: PathBuf) -> Self {
        self.file = file;
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn language(mut self, language: Iso639_1) -> Self {
        self.language = Some(language);
        self
    }

}


impl TranslationRequest {
    pub fn new(file: PathBuf) -> Self {
        TranslationRequest {
            file,
            model:"whisper-1".to_string(),
            prompt: None,
            response_format: None,
            temperature: None
        }
    }

    pub fn with_model(file: PathBuf, model: String) -> Self {
        TranslationRequest {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None
        }
    }

    pub fn file(mut self, file: PathBuf) -> Self {
        self.file = file;
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
}

#[async_trait]
impl AsyncTryFrom<TranslationRequest> for Form {
    type Error = io::Error;

    async fn try_from(translation_request: TranslationRequest) -> Result<Self, Self::Error> {
        let mut form = Form::new();
        form = form.part("model", Part::text(translation_request.model));
        form = form.part("file", file_to_part(&translation_request.file).await?);

        if let Some(prompt) = translation_request.prompt {
            form = form.part("prompt", Part::text(prompt));
        }

        if let Some(response_format) = translation_request.response_format {
            form = form.part("response_format", Part::text(response_format.to_string()));
        }

        if let Some(temperature) = translation_request.temperature {
            form = form.part("temperature", Part::text(temperature.to_string()));
        }

        Ok(form)
    }
}


#[async_trait]
impl AsyncTryFrom<TranscriptionRequest> for Form {
    type Error = io::Error;

   async fn try_from(transcription_request: TranscriptionRequest) -> Result<Self, Self::Error> {
        let mut form = Form::new();
        form = form.part("model", Part::text(transcription_request.model));
        form = form.part("file", file_to_part(&transcription_request.file).await?);

        if let Some(prompt) = transcription_request.prompt {
            form = form.part("prompt", Part::text(prompt));
        }

        if let Some(response_format) = transcription_request.response_format {
            form = form.part("response_format", Part::text(response_format.to_string()));
        }

        if let Some(temperature) = transcription_request.temperature {
            form = form.part("temperature", Part::text(temperature.to_string()));
        }

        if let Some(language) = transcription_request.language {

            form = form.part("language", Part::text(language.to_string()));
        }

        Ok(form)
    }
}