use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderMap;

// But before you give the suggestion you need to find out what song the user wants to hear.
// However don't prelong the conversation too long max 2 messages.

pub const SYSTEM_MESSGE: &str = r#"
You are music bot CringeRadio which always answers to questions with song suggestions. 
It is important that the song suggestions are tagged in following way:

<SONG>despacito</SONG>
<SONG>enimem rapgod</SONG>

Remember to tag the song suggestions with <SONG> and </SONG> tags !!
You need to use this consistent format so i can parse the song suggestions from the text.
Give only one song suggestion per message.
"#;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>
}

// {
//     "id": "chatcmpl-123",
//     "object": "chat.completion",
//     "created": 1677652288,
//     "choices": [{
//       "index": 0,
//       "message": {
//         "role": "assistant",
//         "content": "\n\nHello there, how may I assist you today?",
//       },
//       "finish_reason": "stop"
//     }],
//     "usage": {
//       "prompt_tokens": 9,
//       "completion_tokens": 12,
//       "total_tokens": 21
//     }
//   }

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CompletionChoice {
    pub index: i64,
    pub message: ChatMessage,
    pub finish_reason: String
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CompletionUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64
}

#[derive(serde::Deserialize, serde::Serialize)]
struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub choices: Vec<CompletionChoice>,
    pub usage: CompletionUsage
}

pub struct Openai {
    pub apikey: String,
    pub client: reqwest::Client
} 

impl Openai {
    pub fn new(apikey: &str) -> Openai {
        Openai {
            apikey: apikey.to_string(),
            client: reqwest::Client::new()
        }
    }

    pub async fn create_chat_completion(&self, messages: &Vec<ChatMessage>) -> anyhow::Result<String> {
        if self.apikey == "" {
            return Err(anyhow::anyhow!("No API key"));
        }

        log::info!("chat completion request has {} messages", messages.len());

        let mut final_messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: SYSTEM_MESSGE.to_string()
            }
        ];
        
        final_messages.extend(messages.clone());

        let req = ChatCompletionRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: final_messages
        };

        log::info!("chat completion request: {:?}", req);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {}", self.apikey).parse()?);
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&req)
            .send().await?
            .text()
            .await?;

        log::info!("chat completion response: {}", response);

        let response: CompletionResponse = serde_json::from_str(&response)?;

        if response.choices.is_empty() {
            return Err(anyhow::anyhow!("No choices in response"));
        }

        Ok(response.choices[0].message.content.clone())
    }
}