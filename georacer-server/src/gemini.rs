use serde::{Deserialize, Serialize};
use tracing::trace;

// Request structs
#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<RequestContent>,
}

#[derive(Serialize)]
pub struct RequestContent {
    pub parts: Vec<RequestPart>,
}

#[derive(Serialize)]
pub struct RequestPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<InlineData>,
}

#[derive(Serialize)]
pub struct InlineData {
    pub mime_type: String,
    pub data: String,
}

// Response structs
#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
pub struct Candidate {
    pub content: ResponseContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContent {
    pub parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
pub struct ResponsePart {
    pub text: String,
}

pub async fn is_same_image(image1_b64: &str, image2_b64: &str) -> Result<bool, reqwest::Error> {
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro-vision:generateContent?key={}",
        api_key
    );

    let request = GeminiRequest {
        contents: vec![RequestContent {
            parts: vec![
                RequestPart {
                    text: Some("Are these two images of the same real-world object or location? The images may be from different perspectives or in different lighting. Answer with only 'yes' or 'no'.".to_string()),
                    inline_data: None,
                },
                RequestPart {
                    text: None,
                    inline_data: Some(InlineData {
                        mime_type: "image/jpeg".to_string(),
                        data: image1_b64.to_string(),
                    }),
                },
                RequestPart {
                    text: None,
                    inline_data: Some(InlineData {
                        mime_type: "image/jpeg".to_string(),
                        data: image2_b64.to_string(),
                    }),
                },
            ],
        }],
    };

    let client = reqwest::Client::new();
    let res = client.post(&url).json(&request).send().await?;

    let gemini_response: GeminiResponse = res.json().await?;

    if let Some(candidate) = gemini_response.candidates.get(0) {
        if let Some(part) = candidate.content.parts.get(0) {
            trace!("gemini: {}", &part.text);
            return Ok(part.text.to_lowercase().contains("yes"));
        }
    }

    Ok(false)
}
