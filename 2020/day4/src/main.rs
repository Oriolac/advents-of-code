use std::collections::HashMap;
use std::{fs, env};
use itertools::Itertools;
use std::str::Split;
use std::collections::hash_map::RandomState;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: String = fs::read_to_string(&args[1])
        .expect("Error reading the file");
    let splits: Split<&str> = file.split("\n");
    let f: fn(&str) -> bool = |x| x == "";
    println!("{}", validation(splits.clone(), f, check_validness1));
    println!("{}", validation(splits.clone(), f, check_validness2));
}

fn validation(splits: Split<&str>, f: fn(&str) -> bool, checker: fn(HashMap<&str, &str, RandomState>) -> bool) -> i32 {
    let mut current: String = String::from("");
    let mut res: i32 = 0;
    for line in splits {
        if f(line) {
            let passport = current.split_whitespace()
                .flat_map(|x| x.split(':'))
                .tuples()
                .collect::<HashMap<_, _>>();
            res += match checker(passport) {
                true => 1,
                _ => 0,
            };
            current = String::from("");
        } else {
            current.push(' ');
            current.push_str(line);
        }
    }
    return res;
}

fn check_validness1(passport: HashMap<&str, &str>) -> bool {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let res = fields.iter().all(|x| passport.contains_key(x));
    return res;
}

fn check_validness2(passport: HashMap<&str, &str>) -> bool {
    return match check_validness1(passport.clone()) {
        true => {
            return between(passport.get("byr").expect("Birth year not found"), 1920, 2002) &&
                between(passport.get("iyr").expect("Issue year not found"), 2010, 2020) &&
                between(passport.get("eyr").expect("Expiration year not found"), 2020, 2030) &&
                check_height(passport.get("hgt")) && check_hair(passport.get("hcl")) &&
                check_eye_color(passport.get("ecl")) && check_pid(passport.get("pid"));
        }
        false => false,
    };
}

fn check_pid(option_pid: Option<&&str>) -> bool {
    let pid = option_pid.expect("Pid expected");
    Regex::new(r"\d{9}").unwrap().is_match(pid)
}

fn check_eye_color(option_eye: Option<&&str>) -> bool {
    let eye = option_eye.expect("Eye not found");
    Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap().is_match(eye)
}

fn check_hair(option_hair: Option<&&str>) -> bool {
    let hair = option_hair.expect("Hair not found");
    Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(hair)
}

fn check_height(option_height: Option<&&str>) -> bool {
    let height = option_height.expect("Height not found");
    let regex_cm = Regex::new("...cm").unwrap();
    return match regex_cm.is_match(height) {
        true => {
            let value = height.split_at(3).0.parse::<i32>().unwrap();
            (150 <= value) && (value <= 193)
        }
        false => {
            if !Regex::new("..in").unwrap().is_match(height) {
                return false;
            }
            let value = height.split_at(2).0.parse::<i32>().unwrap();
            (59 <= value) && (value <= 76)
        }
    };
}

fn between(string: &&str, from: i32, to: i32) -> bool {
    let min4 = Regex::new(r"\d{4}").unwrap();
    if !min4.is_match(string) {
        return false;
    }
    let value = string.parse::<i32>().unwrap();
    return from <= value && value <= to;
}
