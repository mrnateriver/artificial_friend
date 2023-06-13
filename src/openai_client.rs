use reqwest::Client;
use serde_json::{json, Value};
use tracing::{error, info};

pub enum OpenAiChatMessage {
    System { text: String },
    User { name: String, text: String },
    Assistant { text: String },
}

pub struct OpenAiClient {
    http: Client,
    api_key: String,
    base_url: String,
}

impl OpenAiClient {
    pub fn new(http: Client, api_key: String) -> Self {
        Self {
            http,
            api_key,
            base_url: "https://api.openai.com".to_string(),
        }
    }

    pub async fn chat(
        &self,
        dialog: impl Iterator<Item = &OpenAiChatMessage>,
        max_tokens: i32,
        temperature: f32,
        frequency_penalty: f32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let api_url = format!("{}/v1/chat/completions", self.base_url);

        let params = json!({
            "model": "gpt-4",
            "max_tokens": max_tokens,
            "temperature": temperature,
            "frequency_penalty": frequency_penalty,
            "messages": dialog.map(|msg| match msg {
                OpenAiChatMessage::System { text } => json!({ "role": "system", "content": text }),
                OpenAiChatMessage::User { name, text } => json!({ "role": "user", "content": text, "name": name }),
                OpenAiChatMessage::Assistant { text } => json!({ "role": "assistant", "content": text }),
            }).collect::<Vec<_>>(),
        });

        info!(?params, "OpenAI request");

        let response = self
            .http
            .post(api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&params)
            .send()
            .await?;

        if response.status().is_success() {
            info!(?response, "OpenAI response");
            let json: Value = response.json().await?;
            let completion = json["choices"][0]["message"]["content"]
                .as_str()
                .ok_or("Failed to parse the completion")?
                .trim()
                .to_string();

            Ok(completion)
        } else {
            error!(?response, "OpenAI error");
            Err(format!("API request failed with status: {}", response.status()).into())
        }
    }
}
