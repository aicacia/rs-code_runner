use serde_json;
use std::io::{self, Result};

use super::{compile, run, BuildOutput, Output};

#[inline]
fn read_stdin() -> Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

#[inline]
fn repl_compile() {
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

            match compile(&mut build_output) {
                Ok(output) => {
                    println!("{}", serde_json::to_string(&Output::from(output)).unwrap());
                    repl_run(build_output)
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
fn repl_run(build_output: BuildOutput) {
    let argv_json = match read_stdin() {
        Ok(string) => string,
        Err(error) => {
            return println!(
                "{}",
                serde_json::to_string(&Output::from(error.to_string())).unwrap()
            )
        }
    };

    match serde_json::from_str(argv_json.trim()) {
        Ok(input) => match run(&build_output, &input) {
            Ok(output) => {
                println!("{}", serde_json::to_string(&Output::from(output)).unwrap());
                repl_run(build_output)
            }
            Err(error) => println!("{}", serde_json::to_string(&Output::from(error)).unwrap()),
        },
        Err(error) => println!(
            "{}",
            serde_json::to_string(&Output::from(error.to_string())).unwrap()
        ),
    }
}

#[inline]
pub fn repl_start() {
    return repl_compile();
}
