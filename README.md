## Simple Rust tool to return Fibonacci Series

This repository contains a Rust library that returns the Fibonacci series up to a user-defined number of terms. The series is returned as a Vec<u32>.

### Usage
Add the following line to your Cargo.toml file to include this library as a dependency:

```
fibonacci = "0.1.0"
```

Use the following code to print the Fibonacci series:

```
extern crate fibonacci;
use fibonacci::fibonacci_series;

fn main() {
    let n = 10;
    let result = fibonacci_series(n);
    println!("The first {} terms of the Fibonacci series are: {:?}", n, result);
}
```

### Contribute
If you would like to contribute to this repository, please fork the repository, make your changes, and create a pull request.
