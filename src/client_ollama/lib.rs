use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

// Client Struct
#[derive(Clone)]
pub struct LLMClient {
    base_url: String,
    http_client: ReqwestClient,
}

#[derive(Debug)]
struct ChatCompletionError(String);

#[derive(Serialize)]
struct ChatContent {
    msgtype: String,
    body: String,
}

impl fmt::Display for ChatCompletionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to get chat completion: {}", self.0)
    }
}

impl Error for ChatCompletionError {}

impl LLMClient {
    // Create a new client instance
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client: ReqwestClient::new(),
        }
    }

    // Main method for retrieving chat completions.
    pub async fn get_chat_completion(
        &self,
        chat_history: &str,
        user_message: &str,
    ) -> Result<String, Box<dyn Error + Send>> {

        let url = format!("{}/chat_completion", self.base_url);
    
        // Request structure
        let request_body = ChatCompletionRequest {
            chat_history: chat_history.to_string(),
            content: ChatContent {
                msgtype: "Text".to_string(),
                body: user_message.to_string(),
            },
        };
    
        // Send request POST to python service
        let response = match self.http_client.post(&url).json(&request_body).send().await {
            Ok(response) => response,
            Err(err) => return Err(Box::new(err)),
        };
    
        if response.status().is_success() {
            let result: ChatCompletionResponse = match response.json().await {
                Ok(result) => result,
                Err(err) => return Err(Box::new(err)),
            };
            Ok(result.response)
        } else {
            Err(Box::new(ChatCompletionError("Failed to get chat completion".into())))

        }
    }

    // Method to obtain text embeddings from the LLM.
    pub async fn get_embedding(&self, message: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/embedding", self.base_url);

        // Request structure
        let request_body = EmbeddingRequest {
            message: message.to_string(),
        };

        // Send request POST to python service
        let response = self
            .http_client
            .post(&url)
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let result: EmbeddingResponse = response.json().await?;
            Ok(result.embedding)
        } else {
            Err("Failed to get embedding".into())
        }
    }
}

// Chat completion Request struct
#[derive(Serialize)]
struct ChatCompletionRequest {
    chat_history: String,
    content: ChatContent, 
}

// Chat Completion Response struct
#[derive(Deserialize)]
struct ChatCompletionResponse {
    response: String,
}

// Embedding Request struct
#[derive(Serialize)]
struct EmbeddingRequest {
    message: String,
}

// Embedding Response struct
#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: String,
}
