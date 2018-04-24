extern crate runner;
extern crate serde;
extern crate serde_json;

use runner::Output;
use std::io;

fn main() {
    let mut input_json = String::new();

    match io::stdin().read_line(&mut input_json) {
        Ok(_) => match serde_json::from_str(&input_json) {
            Ok(input) => match runner::run(&input) {
                Ok(output) => println!("{}", serde_json::to_string(&Output::from(output)).unwrap()),
                Err(error) => println!("{}", serde_json::to_string(&Output::from(error)).unwrap()),
            },
            Err(error) => println!(
                "{}",
                serde_json::to_string(&Output::from(error.to_string())).unwrap()
            ),
        },
        Err(error) => println!(
            "{}",
            serde_json::to_string(&Output::from(error.to_string())).unwrap()
        ),
    }
}
