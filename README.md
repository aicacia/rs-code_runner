# rs-runner

code runner for lanuages

## support

C
Cpp
ECMAScript/JavaScript
Elixir
Python
Ruby
Rust

## Usage

takes json in the form

```json
{
	"language": "rust",
	"files": [
		{
			name: "main.rs",
			content: "fn main() { println!(\"Hello, world!\"); }"
		}
	],
	"stdin": ""
}
```

## Call from Command line

```bash
$ runner "{\"language\":\"rust\",\"files\":[{\"name\":\"main.rs\",\"content\":\"fn main() { println!(\\\"Hello, world\\\"); }\"}],\"stdin\":\"\"}"
"{\"stdout\":\"Hello, world\\n\",\"stderr\":\"\",\"error\":null}"
```
