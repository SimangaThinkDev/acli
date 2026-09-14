use crate::config::get_api_key;
use reqwest::Client;

pub async fn generate_response(
    input: &str,
) -> Result<String, reqwest::Error> {
    // Make a client object
    let client = Client::new();

    // Get API key by calling function
    let api_key = get_api_key().await;

    // Define model -- Using 3.5 due to rate limiting on the later models
    let model = "gemini-3.5-flash";

    // Build Context to make the CLI work

    // TODO: Replace this with a pre-prompt to help navigate CLI Values

    // Create Json Body
    let body = serde_json::json!({
        "model": model,
        "input": input
    });

    // Call Api with context
    let response = client
        .post("https:generativelanguage.googleapis.com/v1beta/interactions")
        .header("x-goog-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        // Now send the request
        .send()
        .await?
        .text()
        .await?;

    return Ok(response)

}
