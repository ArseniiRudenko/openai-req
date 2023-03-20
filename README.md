# openai-api-rust

Refer to tests folder for examples.
Supports all non-obsolete apis at the moment of writing:
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

## Implementation notes
  - For transcriptions and translations currently only json response format is supported.

   