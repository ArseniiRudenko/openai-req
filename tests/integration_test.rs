extern crate openai_api;
use std::fs;
use openai_api::{ByUrlRequest, DownloadRequest, FormRequest, GetRequest, JsonRequest, OpenAiClient};
use openai_api::chat::structs::*;
use serde::Deserialize;
use openai_api::completion::structs::{CompletionRequest};
use openai_api::edit::structs::EditRequest;
use openai_api::embeddings::structs::EmbeddingRequest;
use openai_api::files::structs::{FileDeleteRequest, FileDownloadRequest, FileInfoRequest, FilesResponse, FileUploadRequest};
use openai_api::structs::{Input, ModelsResponse};

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
async fn chat() -> Result<(),anyhow::Error> {
   let client = get_client();
   let messages  = vec!(Message{
     role: Role::User,
     content: "hello!".to_string(),
   });
   let chat_request = ChatRequest::new(messages);
   let response = chat_request.run(&client).await?;
   dbg!(response);
   Ok(())
}


#[tokio::test]
async fn edit()-> Result<(),anyhow::Error> {
    let client = get_client();
    let instruction = "correct spelling";
    let text = "quick blck fox jupms over lazy dog";
    let request = EditRequest::new_text(instruction).set_input(text);
    let response = request.run(&client).await?;
    dbg!(response);
    Ok(())
}

#[tokio::test]
async fn completion()-> Result<(),anyhow::Error> {
    let client = get_client();
    let prompt = Input::String("long long time ago".to_string());
    let completion_request = CompletionRequest::new(prompt);
    let response =
        completion_request.run(&client).await?;
    dbg!(response);
    Ok(())
}

#[tokio::test]
async fn models()-> Result<(),anyhow::Error> {
    let client = get_client();
    let response = ModelsResponse::get(&client).await?;
    dbg!(response);
    Ok(())
}


#[tokio::test]
async fn embeddings()-> Result<(),anyhow::Error> {
    let client = get_client();
    let embedding_request
        = EmbeddingRequest::new("The food was delicious and the waiter...".into());
    let response =
        embedding_request.run(&client).await?;
    dbg!(response);
    Ok(())
}



#[tokio::test]
async fn files() -> Result<(),anyhow::Error> {
    let client = get_client();
    //upload file
    let file = FileUploadRequest::with_str("Cargo.toml","fine-tune");
    let response = file.run(&client).await?;
    dbg!(&response);
    //ist uploaded files
    let files = FilesResponse::get(&client).await?;
    dbg!(files);
    //get info about single file
    let info_request=FileInfoRequest{
        file_id: response.id
    };
    let info= info_request.run(&client).await?;
    dbg!(&info);
    //download file
    let download_request:FileDownloadRequest = info.clone().into();
    download_request.download_to_file(&client,"fine-tune2.json").await?;
    //delete file
    let delete_request:FileDeleteRequest = info.clone().into();
    let delete_result = delete_request.run(&client).await?;
    dbg!(delete_result);
    fs::remove_file("fine-tune2.json")?;
    Ok(())
}