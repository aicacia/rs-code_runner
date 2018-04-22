extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;
extern crate runner;

use clap::App;
use runner::Output;

fn main() {
    let yaml = load_yaml!("cli/en.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(input_json) = matches.value_of("input_json") {
        match serde_json::from_str(input_json) {
            Ok(input) => match runner::run(&input) {
                Ok(output) => println!(
                    "{:?}",
                    serde_json::to_string(&Output::from(output)).unwrap()
                ),
                Err(error) => {
                    println!("{:?}", serde_json::to_string(&Output::from(error)).unwrap())
                }
            },
            Err(error) => println!(
                "{:?}",
                serde_json::to_string(&Output::from(error.to_string())).unwrap()
            ),
        }
    }
}
