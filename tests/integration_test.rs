extern crate openai_api;
use std::fs;
use openai_api::{FormRequest, GetRequest, JsonRequest, OpenAiClient};
use openai_api::chat::structs::*;
use serde::Deserialize;
use openai_api::completion::structs::{CompletionRequest};
use openai_api::edit::structs::EditRequest;
use openai_api::embeddings::structs::EmbeddingRequest;
use openai_api::files::structs::{FilesResponse, FileUploadRequest};
use openai_api::structs::{ApiResponse, Input, ModelsResponse};

#[derive(Deserialize)]
struct Config{
    key: String
}

fn get_client() -> OpenAiClient{
    let key_config=
        fs::read_to_string("key.toml")
            .expect("failed reading config file");
    let openai:Config =
        toml::from_str(&key_config)
            .expect("can't parse config file");

    return  OpenAiClient::new(&openai.key);
}

#[tokio::test]
async fn chat() {
   let client = get_client();
   let messages  = vec!(Message{
     role: Role::User,
     content: "hello!".to_string(),
   });
   let chat_request = ChatRequest::new(messages);
   let response = chat_request.run(&client)
           .await
           .expect("failed contacting api");
   dbg!(response);
}


#[tokio::test]
async fn edit() {
    let client = get_client();
    let instruction = "correct spelling";
    let text = "quick blck fox jupms over lazy dog";
    let request = EditRequest::new_text(instruction).set_input(text);
    let response = request.run(&client)
        .await
        .expect("failed contacting api");
    dbg!(response);
}

#[tokio::test]
async fn completion() {
    let client = get_client();
    let prompt = Input::String("long long time ago".to_string());
    let completion_request = CompletionRequest::new(prompt);
    let response =
        completion_request.run(&client)
        .await
        .expect("failed contacting api");
    dbg!(response);
}

#[tokio::test]
async fn models() {
    let client = get_client();
    let response: ApiResponse<ModelsResponse> = ModelsResponse::get(&client)
        .await
        .expect("failed contacting api");
    dbg!(response);
}


#[tokio::test]
async fn embeddings() {
    let client = get_client();
    let embedding_request
        = EmbeddingRequest::new("The food was delicious and the waiter...".into());
    let response =
        embedding_request.run(&client)
            .await
            .expect("failed contacting api");
    dbg!(response);
}



#[tokio::test]
async fn files() {
    let client = get_client();
    let file = FileUploadRequest::with_str("Cargo.toml","test");
    let response = file.run(&client)
        .await
        .expect("failed contacting api");
    dbg!(response);

}