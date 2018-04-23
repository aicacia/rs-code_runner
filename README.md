# rs-runner

code runner for lanuages

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
			name: "main.rs",
			content: "fn main() { println!(\"Hello, world!\"); }"
		}
	],
	"argv": []
}
```

## Call from Command line

```bash
$ runner "{\"language\":\"rust\",\"files\":[{\"name\":\"main.rs\",\"content\":\"fn main() { println!(\\\"Hello, world\\\"); }\"}],\"argv\":[]}"
{\"stdout\":\"Hello, world\\n\",\"stderr\":\"\",\"error\":null}
```
