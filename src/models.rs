use serde::Deserialize;

#[derive(Deserialize)]
pub struct Interaction {
    pub steps: Vec<Step>,
}

#[derive(Deserialize)]
pub struct Step {
    #[serde(rename = "type")]
    pub step_type: String,

    pub content: Option<Vec<Content>>,
}

#[derive(Deserialize)]
pub struct Content {
    pub text: Option<String>,
}
