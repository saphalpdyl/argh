# argh: Command-Line Argument to HashMap Parsing

A simple Rust library for parsing command-line arguments into a convenient HashMap structure.

## Features:

Easy Argument Mapping: Parses arguments of the form `key:value` or `key` into a HashMap.
Handles Optional Values: Gracefully accommodates both arguments with values and those without.
Flexible Value Access: Retrieves values as Option<&str>, allowing for optional argument handling.

## Usage:

1. Import the function:
```rust
use argh::argh;
```

2. Pass the command-line arguments:
```rust
let mut args: Vec<String> = std::env::args().collect();
let parsed_args = argh(&mut args);
```

3. Access parsed values
```rust 
if let Some(name) = parsed_args.get("name") {
    println!("Hello, {}!", name.unwrap());
}
```

## Example :
```rust 
fn main() {
    let mut args = vec![
        "my_program".to_string(),
        "name:Alice".to_string(),
        "age:30".to_string(),
        "city".to_string(), // Argument without a value
    ];

    let parsed_args = argh(&mut args);
    println!("{:?}", parsed_args); // Output: {"name": Some("Alice"), "age": Some("30"), "city": None}
}
```

## LICENSE
Copyright 2023 Saphal Poudyal

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
