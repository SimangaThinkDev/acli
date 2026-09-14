use reqwest::Client;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = generate_response(
	"Say Yay if you succesfully got this request. Respond only in json {\"Answer\" : \"{Yes/No}\"}"
    ).await?;

    println!("{}", response);

    Ok(())
}

async fn get_api_key() -> String {
    // Load env vars
    dotenv().ok();

    // Get API Key
    // Using the return keyword because I am a decent person
    return env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set")

}

async fn generate_response(
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
