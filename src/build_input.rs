use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildInput {
    pub timeout: f32,
    pub lang: String,
    pub files: HashMap<String, String>,
}
