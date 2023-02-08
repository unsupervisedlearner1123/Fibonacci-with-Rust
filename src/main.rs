extern crate fibonacci;
use fibonacci::fibonacci_series;

fn main() {
    let n = 10;
    let result = fibonacci_series(n);
    println!("The first {} terms of the Fibonacci series are: {:?}", n, result);
}
