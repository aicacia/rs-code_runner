extern crate runner;

macro_rules! run {
    ($lang:expr, $name:expr, $path:expr) => {{
        let lang = $lang;
        let name = $name;

        let input_files = vec![::runner::InputFile {
            name: name.into(),
            content: include_str!($path).to_string(),
        }];

        let input = ::runner::Input {
            language: lang.into(),
            files: input_files,
            argv: Vec::new(),
        };

        match ::runner::run(&input) {
            Ok(output) => {
                if output.error.is_some() {
                    panic!("{:#?}", output);
                }

                assert_eq!(output.stdout, "Hello, world!\n");
            }
            Err(error) => panic!("{:#?}", error),
        }
    }};
}

#[test]
fn c_test() {
    run!("c", "main.c", "snippets/main.c");
}

#[test]
fn cpp_test() {
    run!("cpp", "main.cpp", "snippets/main.cpp");
}

#[test]
fn elixir_test() {
    run!("elixir", "main.ex", "snippets/main.ex");
}

#[test]
fn java_test() {
    run!("java", "Main.java", "snippets/main.java");
}

#[test]
fn ecmascript_test() {
    run!("ecmascript", "main.js", "snippets/main.js");
}

#[test]
fn python_test() {
    run!("python", "main.py", "snippets/main.py");
}

#[test]
fn ruby_test() {
    run!("ruby", "main.rb", "snippets/main.rb");
}

#[test]
fn rust_test() {
    run!("rust", "main.rs", "snippets/main.rs");
}
