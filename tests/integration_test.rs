extern crate openai_api_rust;
use tokio::runtime::Runtime;
use std::fs;
use openai_api_rust::{OpenAiClient, PostClient};
use openai_api_rust::chat::structs::*;
use serde::Deserialize;
use openai_api_rust::completion::structs::{CompletionRequest, Prompt};
use openai_api_rust::edit::structs::EditRequest;

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

#[test]
fn chat() {
   let runtime =  Runtime::new().unwrap();
   let client = get_client();
   let messages  = vec!(Message{
     role: Role::User,
     content: "hello!".to_string(),
   });
   let chat_request = ChatRequest::new(messages);
   let response =
       runtime
           .block_on(client.run(chat_request))
           .expect("failed contacting api");
   dbg!(response);
}


#[test]
fn edit() {
    let runtime =  Runtime::new().unwrap();
    let client = get_client();
    let instruction = "correct spelling";
    let text = "quick blck fox jupms over lazy dog";
    let request = EditRequest::new_text(instruction).set_input(text);
    let response =
    runtime
        .block_on(client.run(request))
        .expect("failed contacting api");
    dbg!(response);
}

#[test]
fn completion() {
    let runtime =  Runtime::new().unwrap();
    let client = get_client();
    let prompt = Prompt::String("long long time ago".to_string());
    let completion_request = CompletionRequest::new(prompt);
    let response =
        runtime
            .block_on(client.run(completion_request))
            .expect("failed contacting api");
    dbg!(response);
}
