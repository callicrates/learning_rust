
// Compute a number in the Fibonacci sequence using recursion. 
// Sequence starts at index 0, where F(0) = 0 and F(1) = 1.
fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

// Compute a number in the Fibonacci sequence using iteration.
// Sequence starts at index 0, where F(0) = 0 and F(1) = 1.
fn fibonacci_iterative(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    // n=40 is where the recursive version starts to be painful in
    // Python.  Let's see how it does in Rust.
    let n = 40; 
    println!("Fibonacci of {} (recursive): {}", n, fibonacci_recursive(n));
    println!("Fibonacci of {} (iterative): {}", n, fibonacci_iterative(n));
}