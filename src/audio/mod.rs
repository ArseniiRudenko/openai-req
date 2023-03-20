use crate::audio::structs::{AudioResponse, TranscriptionRequest, TranslationRequest};
use crate::FormRequest;
pub mod structs;


impl FormRequest<AudioResponse> for TranscriptionRequest{
    const ENDPOINT: &'static str = "/audio/transcriptions";
}

impl FormRequest<AudioResponse> for TranslationRequest{
    const ENDPOINT: &'static str = "/audio/translations";
}