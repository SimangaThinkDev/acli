// src/main.rs
mod request;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = request::generate_response(
	"Say Yes if you succesfully got this request. Respond only in json {\"Answer\" : \"{Yes/No}\"}"
    ).await?;

    println!("{}", response);

    Ok(())
}
