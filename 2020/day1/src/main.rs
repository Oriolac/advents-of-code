use std::env;
use std::fs;
use itertools::Itertools;

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
    let perms: Vec<Vec<i32>> = integers.into_iter().permutations(2).filter(|x| x[0] + x[1] == 2020).collect();
    let solution: Vec<i32> = perms[0].clone();
    println!("{}", solution[0] * solution[1]);

}