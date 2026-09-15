use crate::models::Interaction;

pub fn extract(response: &str) -> Result<String, serde_json::Error> {
    let interaction: Interaction = serde_json::from_str(response)?;

    for step in &interaction.steps {
        if step.step_type == "model_output" {
            if let Some(contents) = &step.content {
                for content in contents {
                    if let Some(text) = &content.text {
                        return Ok(text.clone());
                    }
                }
            }
        }
    }

    Ok(String::new())
}
