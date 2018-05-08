extern crate code_runner;
extern crate serde;
extern crate serde_json;

use code_runner::{BuildOutput, Output};
use std::io::{self, Result};

#[inline]
fn read_stdin() -> Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

#[inline]
fn compile() {
    let build_input_json = match read_stdin() {
        Ok(string) => string,
        Err(error) => {
            return println!(
                "{}",
                serde_json::to_string(&Output::from(error.to_string())).unwrap()
            )
        }
    };

    match serde_json::from_str(build_input_json.trim()) {
        Ok(build_input) => {
            let mut build_output = match BuildOutput::new(&build_input) {
                Ok(build_output) => build_output,
                Err(error) => {
                    return println!("{}", serde_json::to_string(&Output::from(error)).unwrap())
                }
            };

            match code_runner::compile(&mut build_output) {
                Ok(output) => {
                    println!("{}", serde_json::to_string(&Output::from(output)).unwrap());
                    run(build_output)
                }
                Err(error) => {
                    return println!("{}", serde_json::to_string(&Output::from(error)).unwrap())
                }
            }
        }
        Err(error) => {
            return println!(
                "{}",
                serde_json::to_string(&Output::from(error.to_string())).unwrap()
            )
        }
    }
}

#[inline]
fn run(build_output: BuildOutput) {
    let argv_json = match read_stdin() {
        Ok(string) => string,
        Err(error) => {
            return println!(
                "{}",
                serde_json::to_string(&Output::from(error.to_string())).unwrap()
            )
        }
    };

    match serde_json::from_str::<Vec<String>>(argv_json.trim()) {
        Ok(argv) => match code_runner::run(&build_output, &argv) {
            Ok(output) => {
                println!("{}", serde_json::to_string(&Output::from(output)).unwrap());
                run(build_output)
            }
            Err(error) => println!("{}", serde_json::to_string(&Output::from(error)).unwrap()),
        },
        Err(error) => println!(
            "{}",
            serde_json::to_string(&Output::from(error.to_string())).unwrap()
        ),
    }
}

fn main() {
    return compile();
}
