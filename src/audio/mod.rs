use crate::audio::structs::{AudioResponse, TranscriptionRequest, TranslationRequest};
use crate::{FormClient, OpenAiClient};

pub mod structs;


impl<'a> FormClient<'a,TranscriptionRequest,AudioResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/audio/transcriptions";
}

impl<'a> FormClient<'a,TranslationRequest,AudioResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/audio/translations";
}