use anyhow::{Ok, Result};
use askama::Template;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::lyrics::prompt::PromptTemplate;

/**
 * 入参
{
    "model": "qwen-plus",
    "messages": [
        {
            "role": "system",
            "content": "You are a helpful assistant."
        },
        {
            "role": "user",
            "content": "你是谁？"
        }
    ]
}
 */

/**
 * 返回
{
    "choices": [
        {
            "message": {
                "role": "assistant",
                "content": "我是阿里云开发的一款超大规模语言模型，我叫通义千问。"
            },
            "finish_reason": "stop",
            "index": 0,
            "logprobs": null
        }
    ]
}
 */

const BASE_URL: &str =
    "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatQwen {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatQwenResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
}

impl ChatQwen {
    // 调用阿里云通用千问接口翻译文本
    pub async fn exec_translate(text: &str, openai_key: &str) -> Result<String> {
        let context = text;
        let promptTemp = PromptTemplate { context: &context };
        let prompt = promptTemp.render()?;
        // println!("prompt={:?}", prompt);

        let mut messages = vec![Message {
            role: "system".to_string(),
            content: prompt,
        }];

        messages.push(Message {
            role: "user".to_string(),
            content: text.to_string(),
        });


        let chat_qwen = ChatQwen {
            model: "qwen-plus".to_string(),
            messages: messages,
        };
        // println!("chat_qwen={:?}", chat_qwen);
        // println!("chat_qwen_json={}", json!(&chat_qwen));


        // 2、请求API
        let client = Client::new();
        let request = client.request(Method::POST, BASE_URL);

        let resp = request
            .header("Authorization", "Bearer ".to_string() + openai_key)
            .json(&json!(chat_qwen))
            .send()
            .await?;

        // 3、解析响应
        let resp: ChatQwenResponse = resp.json().await?;
        // println!("resp={:?}", resp);

        let resp = resp.choices[0].message.content.clone();
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_exec_chat() {
        let text = "The picture does not fit";
        let openai_key = "sk-510ed600fa2342ffbb88d53931bb70b0";
        let resp = ChatQwen::exec_translate(&text, &openai_key)
            .await
            .unwrap();
        println!("resp={:?}", resp);
    }
}
