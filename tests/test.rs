extern crate code_runner;

macro_rules! test {
    ($test:ident, $lang:expr, $name:expr, $path:expr) => {
        #[test]
        fn $test() {
            let lang = $lang;
            let name = $name;

            let input_files = vec![::code_runner::InputFile {
                name: name.into(),
                content: include_str!($path).to_string(),
            }];

            let input = ::code_runner::Input {
                language: lang.into(),
                files: input_files,
                argv: Vec::new(),
            };

            match ::code_runner::run(&input) {
                Ok(output) => {
                    if output.error.is_some() {
                        panic!("{:#?}", output);
                    }

                    assert_eq!(output.stdout, "Hello, world!\n");
                }
                Err(error) => panic!("{:#?}", error),
            }
        }
    };
}

test!(c_test, "c", "main.c", "snippets/main.c");
test!(cpp_test, "cpp", "main.cpp", "snippets/main.cpp");
test!(elixir_test, "elixir", "main.ex", "snippets/main.ex");
test!(java_test, "java", "Main.java", "snippets/Main.java");
test!(ecmascript_test, "ecmascript", "main.js", "snippets/main.js");
test!(python_test, "python", "main.py", "snippets/main.py");
test!(ruby_test, "ruby", "main.rb", "snippets/main.rb");
test!(rust_test, "rust", "main.rs", "snippets/main.rs");
