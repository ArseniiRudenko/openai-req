use crate::moderations::structs::{ModerationRequest, ModerationResponse};
use crate::{OpenAiClient, JsonRequest};

pub mod structs;


impl JsonRequest<ModerationResponse> for ModerationRequest{
    const ENDPOINT: &'static str = "/moderations";
}