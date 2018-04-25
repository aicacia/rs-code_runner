extern crate runner;
extern crate serde;
extern crate serde_json;

use runner::Output;
use std::io;

fn main() -> io::Result<()> {
    let mut input_json = String::new();

    // Docker writes an empty string to stdin
    while input_json.trim().is_empty() {
        input_json.clear();
        io::stdin().read_line(&mut input_json)?;
    }

    match serde_json::from_str(input_json.trim()) {
        Ok(input) => match runner::run(&input) {
            Ok(output) => println!("{}", serde_json::to_string(&Output::from(output)).unwrap()),
            Err(error) => println!("{}", serde_json::to_string(&Output::from(error)).unwrap()),
        },
        Err(error) => println!(
            "{}",
            serde_json::to_string(&Output::from(error.to_string())).unwrap()
        ),
    }

    Ok(())
}
