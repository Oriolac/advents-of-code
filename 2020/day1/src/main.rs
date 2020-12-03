use std::env;
use std::fs;
use itertools::Itertools;
use reduce::Reduce;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: ./day1 <permutations> <filename>");
        std::process::exit(1);
    }
    let p = args[1].parse::<i32>();
    let contents: String = fs::read_to_string(&args[2])
        .expect("File not found");
    let words: Vec<&str> = contents.split_whitespace().collect();
    let integers: Vec<i32> = words.iter()
        .map(|x| x.parse::<i32>().unwrap()).rev().collect();
    let perms: Option<Vec<i32>> = integers.into_iter()
        .permutations(p)
        .filter(|x| x.into_iter().sum::<i32>() == 2020).next();
    match perms {
        Some(x) => {
            println!("{}", x.into_iter().reduce(|a, b| a * b).unwrap());
        },
        None => println!("Not found"),
    }
}