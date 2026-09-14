// src/lib.rs
pub mod config;
pub mod gemini;
pub mod models;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {

    let response = gemini::generate_response(
	"Say Yes if you succesfully got this request, Respond only in JSON {\"Answer\":\"{Yes/No}\"}"
    ).await?;

    println!("{}", response);

    Ok(())
}
