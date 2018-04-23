extern crate runner;
extern crate serde;
extern crate serde_json;

use runner::Output;
use std::env::args;

fn main() {
    let argv = args().collect::<Vec<String>>();

    match argv.get(1) {
        Some(input_json) => match serde_json::from_str(input_json) {
            Ok(input) => match runner::run(&input) {
                Ok(output) => println!("{}", serde_json::to_string(&Output::from(output)).unwrap()),
                Err(error) => println!("{}", serde_json::to_string(&Output::from(error)).unwrap()),
            },
            Err(error) => println!(
                "{}",
                serde_json::to_string(&Output::from(error.to_string())).unwrap()
            ),
        },
        None => println!(
            "{}",
            serde_json::to_string(&Output::from("Invalid Argument")).unwrap()
        ),
    }
}
