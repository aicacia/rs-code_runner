extern crate code_runner;
extern crate serde_json;

use code_runner::{compile, run, BuildInput, BuildOutput, Error, Input};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

macro_rules! test {
    ($test:ident, $lang:expr, $name:expr, $path:expr) => {
        #[test]
        fn $test() {
            let lang = $lang;
            let name = $name;

            let mut files = HashMap::new();

            files.insert(name.into(), include_str!($path).to_string());

            let build_input = BuildInput {
                timeout: 10.0,
                lang: lang.into(),
                files: files,
            };
            let mut build_output = BuildOutput::new(&build_input).unwrap();

            match compile(&mut build_output) {
                Ok(_compile_output) => match run(&build_output, &Input::new(1.0, &[])) {
                    Ok(output) => {
                        if output.error.is_some() {
                            panic!("{:#?}", output);
                        }

                        assert_eq!(output.stdout, "Hello, world!\n");
                    }
                    Err(error) => panic!("{:#?}", error),
                },
                Err(error) => panic!("{:#?}", error),
            }
        }
    };
}

test!(c_test, "c", "main.c", "snippets/main.c");
test!(cpp_test, "cpp", "main.cpp", "snippets/main.cpp");
test!(elixir_test, "elixir", "main.ex", "snippets/main.ex");
test!(golang_test, "golang", "main.go", "snippets/main.go");
test!(java_test, "java", "Main.java", "snippets/Main.java");
test!(ecmascript_test, "ecmascript", "main.js", "snippets/main.js");
test!(lua_test, "lua", "main.lua", "snippets/main.lua");
test!(python_test, "python", "main.py", "snippets/main.py");
test!(ruby_test, "ruby", "main.rb", "snippets/main.rb");
test!(rust_test, "rust", "main.rs", "snippets/main.rs");

macro_rules! test_repl {
    ($test:ident, $lang:expr, $name:expr, $path:expr) => {
        #[test]
        fn $test() {
            let lang = $lang.to_owned();
            let name = $name.to_owned();

            Command::new("cargo")
                .arg("build")
                .output()
                .expect("failed to start binary");

            let mut child = Command::new("./target/debug/code_runner")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to start binary");

            child.stdin.as_mut().map(|stdin| {
                let mut files = HashMap::new();

                files.insert(name, include_str!($path).to_string());

                let build_input = BuildInput {
                    timeout: 10.0,
                    lang: lang,
                    files: files,
                };
                let build_input_json = serde_json::to_string(&build_input).unwrap();

                let _ = stdin.write_all(build_input_json.as_bytes());
                let _ = stdin.write_all(b"\n");
                let _ = stdin.flush();
            });
            let result = child
                .stdout
                .as_mut()
                .map(|stdout| {
                    let mut reader = BufReader::new(stdout);
                    let mut string = String::new();
                    let _ = reader.read_line(&mut string);
                    string
                })
                .unwrap();

            assert_eq!(result, "{\"stdout\":\"\",\"stderr\":\"\",\"error\":null}\n");

            child.stdin.as_mut().map(|stdin| {
                let input = Input {
                    timeout: 1.0,
                    argv: Vec::new(),
                };
                let input_json = serde_json::to_string(&input).unwrap();

                let _ = stdin.write_all(input_json.as_bytes());
                let _ = stdin.write_all(b"\n");
                let _ = stdin.flush();
            });
            let result = child
                .stdout
                .as_mut()
                .map(|stdout| {
                    let mut reader = BufReader::new(stdout);
                    let mut string = String::new();
                    let _ = reader.read_line(&mut string);
                    string
                })
                .unwrap();

            let _ = child.kill();

            assert_eq!(
                result,
                "{\"stdout\":\"Hello, world!\\n\",\"stderr\":\"\",\"error\":null}\n"
            );
        }
    };
}

test_repl!(c_repl_test, "c", "main.c", "snippets/main.c");
test_repl!(cpp_repl_test, "cpp", "main.cpp", "snippets/main.cpp");
test_repl!(elixir_repl_test, "elixir", "main.ex", "snippets/main.ex");
test_repl!(golang_repl_test, "golang", "main.go", "snippets/main.go");
test_repl!(java_repl_test, "java", "Main.java", "snippets/Main.java");
test_repl!(
    ecmascript_repl_test,
    "ecmascript",
    "main.js",
    "snippets/main.js"
);
test_repl!(lua_repl_test, "lua", "main.lua", "snippets/main.lua");
test_repl!(python_repl_test, "python", "main.py", "snippets/main.py");
test_repl!(ruby_repl_test, "ruby", "main.rb", "snippets/main.rb");
test_repl!(rust_repl_test, "rust", "main.rs", "snippets/main.rs");

#[test]
fn timeout_test() {
    let mut files = HashMap::new();

    files.insert(
        "main.js".to_owned(),
        "setTimeout(function() { console.log(\"Hello, world!\"); }, 1000);".to_owned(),
    );

    let build_input = BuildInput {
        timeout: 60.0,
        lang: "javascript".to_owned(),
        files: files,
    };
    let mut build_output = BuildOutput::new(&build_input).unwrap();

    match compile(&mut build_output) {
        Ok(_compile_output) => match run(&build_output, &Input::new(0.0, &[])) {
            Ok(output) => {
                panic!("Should return Error::Timeout returned {:#?}", output);
            }
            Err(error) => assert_eq!(error, Error::Timeout),
        },
        Err(error) => panic!("Should return Error::Timeout returned {:#?}", error),
    }
}
