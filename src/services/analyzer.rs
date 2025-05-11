use std::error::Error;
use std::fmt;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;
use lazy_static::lazy_static;
use std::env;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::services::utils::normalize_fancy_text;

lazy_static! {
    static ref DEEPSEEK_TOKEN: String = env::var("AI_TOKEN_DEEPSEEK").expect("AI_TOKEN_DEEPSEEK not found in .env");
    static ref GEMINI_TOKEN: String = env::var("AI_TOKEN_GEMINI").expect("AI_TOKEN_GEMINI not found in .env");
    static ref AI_SERVICE: String = env::var("AI_SERVICE").unwrap_or("deepseek".to_string());
    static ref AI_PROMPT: String = fs::read_to_string("ai_prompt.txt").expect("Failed to read ai_prompt.txt");
}


#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub comment: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeResponse {
    spam: bool,
    keyword: String,
    confidence: f64,
}

lazy_static! {
    static ref KEYWORD_CACHE: Mutex<HashMap<String, f64>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError(format!("IO error: {}", err))
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> Self {
        CustomError(format!("Reqwest error: {}", err))
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> Self {
        CustomError(format!("Serde JSON error: {}", err))
    }
}

impl From<&str> for CustomError {
    fn from(err: &str) -> Self {
        CustomError(format!("Generic error: {}", err))
    }
}

fn remove_all_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub async fn analyze_comment(comment: &str) -> Result<AnalyzeResponse, CustomError> {
    // Check if the keyword exists in the comment
    let normalized_comment = normalize_fancy_text(comment);
    let sanitzed_comment = remove_all_whitespace(&normalized_comment);
    {
        let cache = KEYWORD_CACHE.lock().unwrap();
        for (keyword, confidence) in cache.iter() {
            // Check if the keyword is present in the normalized comment
            // Use the normalized comment for comparison
            // This is a simple substring check, you might want to use a more sophisticated method
            // depending on your requirements
            // For example, you could use regex or a more complex string matching algorithm
            // to check for variations of the keyword
            // For now, we will just check if the keyword is a substring of the comment
            // Normalize the keyword as well
            if sanitzed_comment.contains(&keyword.to_uppercase()) {
                println!("Keyword found in cache: {} with confidence {}", keyword, confidence);
                return Ok(AnalyzeResponse {
                    spam: true,
                    keyword: keyword.clone(),
                    confidence: *confidence,
                });
            }
        }
    }

    let client = reqwest::Client::new();
    let api_response: String;
    println!("Normalized comment: {}", normalized_comment);
    if AI_SERVICE.to_lowercase() == "gemini" {
        // Gemini API call
        // Body : {"system_instruction":{"parts":[{"text":"AI_PROMPT"}]},"contents":[{"parts":[{"text":"COMMENT"}]}],"generationConfig":{"stopSequences":["\n"],"temperature":0.2,"maxOutputTokens":50,"topP":0.5,"topK":3}}
        let response = client
            .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-lite:generateContent")
            .header("Content-Type", "application/json")
            .query(&[("key", GEMINI_TOKEN.clone())])
            .body(format!(r#"{{"system_instruction":{{"parts":[{{"text":"{}"}}]}},
                "contents":[{{"parts":[{{"text":"{}"}}]}}],
                "generationConfig":{{"stopSequences":["\n"],"temperature":0.2,"maxOutputTokens":50,"topP":0.5,"topK":3}}}}"#, AI_PROMPT.clone(), normalized_comment))
            .send()
            .await?;

        api_response = response.text().await?;
        println!("Gemini API response: {}", api_response);
    } else {
        // DeepSeek API call
        let response = client
            .post("https://api.deepseek.com/chat/completions")
            .header("authorization", format!("Bearer {}", DEEPSEEK_TOKEN.clone()))
            .header("content-type", "application/json")
            .body(format!(r#"{{"model": "deepseek-chat","messages": [{{"role": "system","content": "{}"}},{{"role": "user","content": "{}"}}],"stream": false,"max_tokens": 50}}"#, AI_PROMPT.clone(), normalized_comment))
            .send()
            .await?;

        api_response = response.text().await?;
        println!("DeepSeek API response: {}", api_response);
    }

    // Parse the response and extract the spam classification, keyword, and confidence score
    // This is a placeholder, replace with actual parsing logic
    // DeepSeek API response: {"id":"ca613938-7188-45af-ac94-476df9017341","object":"chat.completion","created":1746897836,"model":"deepseek-chat","choices":[{"index":0,"message":{"role":"assistant","content":"1,MANDALIKA77,0.95"},"logprobs":null,"finish_reason":"stop"}],"usage":{"prompt_tokens":279,"completion_tokens":11,"total_tokens":290,"prompt_tokens_details":{"cached_tokens":0},"prompt_cache_hit_tokens":0,"prompt_cache_miss_tokens":279},"system_fingerprint":"fp_8802369eaa_prod0425fp8"}
    // Gemini API response: {"candidates":[{"content":{"parts":[{"text":"1,KEYWORD,0.95"}]},"finishReason":"STOP","index":0,"safetyRatings":[{"category":"HARM_CATEGORY_HARASSMENT","probability":"NEGLIGIBLE"},{"category":"HARM_CATEGORY_HATE_SPEECH","probability":"NEGLIGIBLE"},{"category":"SEXUALLY_EXPLICIT","probability":"NEGLIGIBLE"},{"category":"DANGEROUS_CONTENT","probability":"NEGLIGIBLE"}]}]}
    let parsed_response: serde_json::Value = serde_json::from_str(&api_response)?;
    let mut parts: Vec<&str>;
    let mut spam: i32 = 0;
    let mut keyword = String::new();
    let mut confidence: f64 = 0.0;

    if AI_SERVICE.to_lowercase() == "gemini" {
        let candidates = parsed_response["candidates"].as_array().ok_or("Invalid response format")?;
        if candidates.is_empty() {
            return Err("No candidates found in response".into());
        }
        let content = &candidates[0]["content"]["parts"][0]["text"].as_str().replace("\n").ok_or("Invalid content format")?;
        parts = content.split(',').collect();

        if parts.len() != 3 {
            return Err("Invalid content format".into());
        }

        spam = parts[0].parse().map_err(|_| "Invalid spam value")?;
        keyword = parts[1].to_string();
        confidence = parts[2].parse().map_err(|_| "Invalid confidence value")?;
    } else {
        let choices = parsed_response["choices"].as_array().ok_or("Invalid response format")?;
        if choices.is_empty() {
            return Err("No choices found in response".into());
        }
        let message = &choices[0]["message"];
        let content = message["content"].as_str().ok_or("Invalid content format")?;
        parts = content.split(',').collect();

        if parts.len() != 3 {
            return Err("Invalid content format".into());
        }

        spam = parts[0].parse().map_err(|_| "Invalid spam value")?;
        keyword = parts[1].to_string();
        confidence = parts[2].parse().map_err(|_| "Invalid confidence value")?;
    }

    println!("Parsed response: spam: {}, keyword: {}, confidence: {}", spam, keyword, confidence);

    let mut keyword_cache_lock = KEYWORD_CACHE.lock().unwrap();
    keyword_cache_lock.insert(keyword.clone(), confidence);

    Ok(AnalyzeResponse {
        spam : spam == 1,
        keyword,
        confidence,
    })
}
