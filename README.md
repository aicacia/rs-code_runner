# rs-code_runner

code code_runner for languages

## support

*   C
*   Cpp
*   ECMAScript/JavaScript
*   Elixir
*   Python
*   Ruby
*   Rust

## Usage

takes json in the form

```json
{
	"language": "rust",
	"files": [
		{
			"name": "main.rs",
			"content": "fn main() { println!(\"Hello, world!\"); }"
		}
	],
	"argv": []
}
```

## Call from Command line

### Rust

```bash
$ code_runner
{"language": "rust", "files": [{"name": "main.rs","content": "fn main() { println!(\"Hello, world\"); }"}], "argv": []}
{"stdout":"Hello, world\n","stderr":"","error":null}
```

### ECMAScript/JavaScript

```bash
$ code_runner
{"language": "ecmascript", "files": [{"name": "main.js","content": "console.log(\"Hello, world!\");"}], "argv": []}
{"stdout":"Hello, world\n","stderr":"","error":null}
```
