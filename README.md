# rs-code_runner

code runner for languages

## support

*   C
*   Cpp
*   ECMAScript/JavaScript
*   Elixir
*   Go
*   Java
*   Lua
*   Python
*   Ruby
*   Rust

## Usage

takes json in the form

```json
{
	"lang": "rust",
	"files": {
		"main.rs": "fn main() { println!(\"Hello, world!\"); }"
	}
}
```

## Call from Command line

### Rust

```bash
$ code_runner
{"lang": "rust", "files": {"main.rs": "fn main() { let args = ::std::env::args().collect::<Vec<String>>(); println!(\"{}, {}\", args[1], args[2]); }"}}
{ "timeout": 1.0, "argv": ["Hello", "world!"] }
{"stdout":"Hello, world\n","stderr":"","error":null}
^C
```

### ECMAScript/JavaScript

```bash
$ code_runner
{"lang": "ecmascript", "files": {"main.js": "console.log(process.argv.slice(2).join(\", \"));"}}
{ "timeout": 1.0, "argv": ["Hello", "world!"] }
{"stdout":"Hello, world\n","stderr":"","error":null}
^C
```
