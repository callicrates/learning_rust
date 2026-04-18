// This requires 'cargo add text_io'
#[macro_use] extern crate text_io;

fn collatz(start: i64) -> i64 {
    let mut x = start;
    let mut steps: i64 = 0;

    while x != 1 {
        if (x % 2) == 0 {
            x = x / 2;
        } else {
            x = 3*x + 1;
        }
        steps += 1;
    }
    steps
}

fn main() {
    println!("Please enter an integer.");
    let line: String = read!("{}\n");
    let start_num: i64 = line.parse().unwrap();

    println!("Computing Collatz sequence length for {}.", start_num);    
    let seq_length = collatz(start_num);
    println!("Collatz sequence for {} reached 1 after {} steps.", start_num, seq_length)
}
