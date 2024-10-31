# Ruston

Ruston is a programming language that compiles to Python. It is designed to provide an easier syntax based on Rust, making it more accessible for developers.

## Overview

Ruston aims to combine the power and performance of Rust with the simplicity and flexibility of Python. By compiling Ruston code to Python, developers can leverage the extensive Python ecosystem while enjoying the benefits of Rust's syntax and features.

## Features

- **Easy Syntax**: Ruston offers a simplified syntax inspired by Rust, making it easier to write and read code.
- **Python Compatibility**: Ruston code is compiled to Python, allowing seamless integration with existing Python libraries and frameworks.
- **Virtual Machine**: Soon, Ruston will have its own virtual machine, providing the possibility to compile it to Python or run it in its own virtual machine.

## Getting Started

To get started with Ruston, follow these steps:

1. Clone the repository: `git clone https://github.com/time9683/ruston.git`
2. Navigate to the project directory: `cd ruston`
3. Build the project: `cargo build`
4. Run the Ruston compiler: `cargo run -- path/to/your/file.rstn`

## Example

Here is a simple example of Ruston code:

```ruston
fn main() {
    let message = "Hello, Ruston!";
    println(message);
}
```

This code will be compiled to Python and produce the following output:

```python
def main():
    message = "Hello, Ruston!"
    print(message)

main()
```

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
