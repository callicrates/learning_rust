// This code uses the instructor's error-handling example as a 
// starting point.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

fn cat(filename: &str) -> Result<i32, io::Error> {
    let infile = File::open(filename);
    match infile {
        Err(error) => {
            return Err(error);
        },
        Ok(open_file) => {
            let mut line_count = 0;
            let reader = BufReader::new(open_file);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        line_count += 1;
                        println!("{}", line)
                    },
                    Err(error) => {
                        return Err(error)
                    }
                }
            }
            return Ok(line_count)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} filename", args[0]);
        return;
    }

    let files_to_print = &args[1..];
    for filename in files_to_print {
        println!("About to print {}", filename);
        let result = cat(&filename);
        match result {
            Err(error) => {
                panic!("Error on {}: {}", filename, error);
            },
            _ => {
                println!("Continuing to next file.");
            }
            
        }
    }
}