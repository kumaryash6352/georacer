use gemini_rust::{Gemini, Model};
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

fn parse_data_url(s: &str) -> (String, String) {
    if let Some(prefix) = s.strip_prefix("data:") {
        if let Some(idx) = prefix.find(',') {
            let meta = &prefix[..idx]; // e.g. image/jpeg;base64
            let data = &prefix[idx + 1..];
            let mime = meta.split(';').next().unwrap_or("image/jpeg").to_string();
            return (mime, data.to_string());
        }
    }
    ("image/jpeg".to_string(), s.to_string())
}

pub async fn is_same_image(image1_b64: &str, image2_b64: &str) -> Result<bool, reqwest::Error> {
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let client = Gemini::with_model(api_key, Model::Gemini25FlashLite).unwrap();

    let (mime1, data1) = parse_data_url(image1_b64);
    let (mime2, data2) = parse_data_url(image2_b64);

    let response = client.generate_content()
        .with_system_instruction("Are these two images of the same real-world object or location?           \
                                 The images may be from very different perspectives or in different lighting.   \
                                 Be very lenient with what constitutes as the \"same\".                    \
                                 Answer with only 'yes' or 'no'.".to_string())
        .with_inline_data(&data1, &mime1)
        .with_inline_data(&data2, &mime2)
        .execute().await.unwrap();

    trace!("{}", response.text());
    Ok(response.text().to_lowercase().contains("yes"))
}
