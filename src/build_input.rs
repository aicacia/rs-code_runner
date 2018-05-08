use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildInput {
    pub lang: String,
    pub files: HashMap<String, String>,
}
