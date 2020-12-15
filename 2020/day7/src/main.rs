use std::{env, fs};
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();
    let lines = file
        .split("\n")
        .map(|x| {
            let tuple: Vec<&str> = x.split("contain").collect();
            return (tuple[0].split(" ").take(2).join(" "), get_bags(tuple[1]));
        }).collect_vec();
    part1(&lines);
}

fn part1(lines: &Vec<(String, Vec<(i32, String)>)>) {
    let mut stack = Vec::new();
    stack.push("shiny gold");
    let mut generated = Vec::new();
    while !stack.is_empty() {
        let bag = stack.pop().unwrap();
        generated.push(bag);
        for elem in lines.iter()
            .filter(|&x| !x.1.iter()
                .filter(|&x| x.1 == String::from(bag))
                .collect_vec()
                .is_empty()) {
            if !generated.contains(&elem.0.as_str()) && !stack.contains(&elem.0.as_str()) {
                stack.push(elem.0.as_str());
            }
        }
    }
    println!("{}", generated.len() - 1);
}

fn find(lines: Vec<(String, Vec<(i32, String)>)>, bag: &str) -> Vec<(i32, String)> {
    return lines.filter(|x| x.0 == String::from(bag)).1;
}

fn get_bags(line: &str) -> Vec<(i32, String)> {
    let b = line.split(",").map(|x| {
        let words: Vec<&str> = x.split(" ").filter(|&x| x != "").collect_vec();
        let num: (i32, String) = words[0].parse::<i32>().map_or((0, String::new()), |x| (x, words[1..3].join(" ")));
        return num;
    }).collect_vec();
    return b;
}


