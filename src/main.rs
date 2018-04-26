extern crate code_runner;
extern crate serde;
extern crate serde_json;

use code_runner::Output;
use std::io::{self, Result};

#[inline]
fn read_stdin() -> Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn main() {
    let input_json = match read_stdin() {
        Ok(string) => string,
        Err(error) => {
            return println!(
                "{}",
                serde_json::to_string(&Output::from(error.to_string())).unwrap()
            )
        }
    };

    match serde_json::from_str(input_json.trim()) {
        Ok(input) => match code_runner::run(&input) {
            Ok(output) => println!("{}", serde_json::to_string(&Output::from(output)).unwrap()),
            Err(error) => println!("{}", serde_json::to_string(&Output::from(error)).unwrap()),
        },
        Err(error) => println!(
            "{}",
            serde_json::to_string(&Output::from(error.to_string())).unwrap()
        ),
    }
}
