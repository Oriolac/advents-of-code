use std::env;
use std::fs;
use std::process::exit;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./day1 <filename>");
        std::process::exit(1);
    }
    let contents: String = fs::read_to_string(&args[1])
        .expect("File not found");
    let words: Vec<&str> = contents.split_whitespace().collect();
    let integers: Vec<i32> = words.iter().map(|x| x.parse::<i32>().unwrap()).rev().collect();
    for a in integers.iter() {
        for b in integers.iter() {
            if a + b == 2020 {
                println!("{}", a * b);
                exit(0);
            }
        }
    }
}