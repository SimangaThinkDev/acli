// src/lib.rs
pub mod config;
pub mod gemini;
pub mod models;
pub mod input;
pub mod extract_content;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {

    // Let's try prompting the user using predfined functions
    let mut prompt = input::get_str(Some("Good Day How can I help you...\n>>"));
    let mut context = String::new();

    while prompt != "exit" {
        // Full User prompt
	let full_prompt = format!("{}\nContext so far\n{}", prompt, context);

	let response = gemini::generate_response(
	    &full_prompt
        ).await?;

	let clean_response = extract_content::extract(&response)?;

	println!(">> {}\n\nType `exit` to leave the program (case sensitive)", clean_response);
        
        // Add user prompt to context
	context.push_str(&format!("\n{}", prompt));
	// Add model response to context
	context.push_str(&format!("\n{}", clean_response));

	prompt = input::get_str(None);
    }

    // TODO: Parse output


// OLD TEST CODE
//    let response = gemini::generate_response(
//	"Say Yes if you succesfully got this request, Respond only in JSON {\"Answer\":\"{Yes/No}\"}"
//    ).await?;

//    println!("{}", response);

    Ok(())
}
