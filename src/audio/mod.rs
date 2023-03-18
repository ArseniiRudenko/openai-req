use crate::audio::structs::{AudioResponse, TranscriptionRequest, TranslationRequest};
use crate::{FormClient, OpenAiClient};
use crate::structs::ApiResponse;

pub mod structs;


impl FormClient<TranscriptionRequest,AudioResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/audio/transcriptions";
}

impl FormClient<TranslationRequest,AudioResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/audio/translations";
}