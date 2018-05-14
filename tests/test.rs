extern crate code_runner;

macro_rules! test {
    ($test:ident, $lang:expr, $name:expr, $path:expr) => {
        #[test]
        fn $test() {
            let lang = $lang;
            let name = $name;

            let mut files = ::std::collections::HashMap::new();

            files.insert(name.into(), include_str!($path).to_string());

            let build_input = ::code_runner::BuildInput {
                lang: lang.into(),
                files: files,
            };
            let mut build_output = ::code_runner::BuildOutput::new(&build_input).unwrap();

            match ::code_runner::compile(&mut build_output) {
                Ok(_compile_output) => {
                    match ::code_runner::run(&build_output, &::code_runner::Input::new(5.0, &[])) {
                        Ok(output) => {
                            if output.error.is_some() {
                                panic!("{:#?}", output);
                            }

                            assert_eq!(output.stdout, "Hello, world!\n");
                        }
                        Err(error) => panic!("{:#?}", error),
                    }
                }
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
