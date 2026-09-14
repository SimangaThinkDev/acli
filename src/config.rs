use dotenvy::dotenv;
use std::env;

pub async fn get_api_key() -> String {
    // Load env vars
    dotenv().ok();

    // Get API Key
    // Using the return keyword because I am a decent person
    return env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set")

}
