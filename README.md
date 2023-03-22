# openai-req
OpenAI API client implemented using tokio and reqwest.
Refer to tests folder in the repo or to code comments for usage examples.

## Usage.
- Import library
   ```toml
   [dependencies]
   openai-req="1"
   ```
- First you will need to construct client. Minimal client only requires API key. Here is a simple example with reading key from toml file.

   Let's say you have key in key.toml file, that looks like this:
   ```toml
   key = "{YOUR_KEY}"
   ```
   Function that constructs client by reading key.toml will look like this:
   ```rust
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
   ```
- Next you generally construct Request structure, and trigger run method, passing client reference to it.
  here is an example for chat completion:
  ```rust
  async fn chat() -> Result<ChatSuccess,anyhow::Error> {
      let client = get_client();
      let messages  = vec!(Message{
        role: Role::User,
        content: "hello!".to_string(),
      });
      let chat_request = ChatRequest::new(messages);
      Ok(chat_request.run(&client).await?)
   }
  ```
- For get requests that do not take any parameters, you generally call static `get` function on response type. 
Usually that type is called  *Something*ListResponse:
  ```rust
  async fn models() -> Result<ModelListResponse,anyhow::Error> {
    let client = get_client();
    Ok(ModelListResponse::get(&client).await?)
  }
  ```
- And finally, for download file requests, request type will have two methods:

   `async fn download_to_file(&self, client:&OpenAiClient, target_path:&str) -> Result<()>` 

   and

   `async fn download(&self, client:&OpenAiClient) -> Result<Pin<Box<dyn Stream<Item=Result<Bytes, reqwest::Error>>>>>`

   First one takes path on local fs, and creates downloaded file there. 
   Second one just returns async data stream, and lets you figure out what to do with it.
   
   Here is an example:
   ```rust
    async fn file_download() -> Result<(),anyhow::Error> {
       let client = get_client();
       let files = FileListResponse::get(&client).await?;
       let info = files.data.first().ok_or(anyhow!("No files available"))?;
       let download_request: FileDownloadRequest = info.clone().into();
       download_request.download_to_file(&client, "fine-tune2.json").await
    }  
   ```
  
## Supported APIs:
1. Models:
    - List
    - Retrieve
2. Completions:
    - Create
3. Chat:
    - Create
4. Edits:
    - Create
5. Images:
    - Create
    - Create edit
    - Create variation
6. Embeddings:
    - Create
7. Audio
    - Create transcription
    - Create translation
8. Files
    - List
    - Upload
    - Delete
    - Retrieve file
    - Retrieve file content
9. Fine-tunes
    - Create fine-tune
    - List fine-tunes
    - Retrieve fine-tune
    - List fine-tune events
    - Delete fine-tune model
10. Moderations
    - Create moderation