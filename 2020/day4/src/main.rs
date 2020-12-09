use std::collections::HashMap;
use std::{fs, env};
use itertools::Itertools;
use std::str::Split;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file: String = fs::read_to_string(&args[1])
        .expect("Error reading the file");
    let splits: Split<&str> = file.split("\n");
    let f: fn(&str) -> bool = |x| x == "";
    println!("{}", validation(splits.clone(), f));
}

fn validation(splits: Split<&str>, f: fn(&str) -> bool) -> i32 {
    let mut current: String = String::from("");
    let mut res: i32 = 0;
    for line in splits {
        if f(line) {
            let passport = current.split_whitespace()
                .flat_map(|x| x.split(':'))
                .tuples()
                .collect::<HashMap<_, _>>();
            res += match check_validness(passport) {
                true=> 1,
                _ => 0,
            };
            current = String::from("");
        } else {
            current.push(' ');
            current.push_str(line); }
    }
    return res;
}

fn check_validness(passport: HashMap<&str, &str>) -> bool {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let res = fields.iter().all(|x| passport.contains_key(x));
    return  res;
}