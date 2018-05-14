#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub timeout: f32,
    pub argv: Vec<String>,
}

impl Input {
    #[inline]
    pub fn new(timeout: f32, argv: &[&str]) -> Self {
        Input {
            timeout: timeout,
            argv: argv.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}
