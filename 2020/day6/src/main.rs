use std::{env, fs};
use std::collections::HashSet;
use reduce::Reduce;
use std::collections::hash_set::Intersection;
use std::iter::FromIterator;
use std::convert::TryFrom;
use std::borrow::BorrowMut;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file= fs::read_to_string(&args[1]).unwrap();
    let lines = file.split("\n");
    let groups = grouping(lines.clone().collect());
    println!("{}", groups.iter().sum::<i32>());
    let match_answers: Vec<i32> = matching(lines.clone().collect());
    println!("{}", match_answers.iter().sum::<i32>());
}

fn matching(lines: Vec<&str>) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();
    for groups in lines.split(|&x| x == "") {
        println!("{:?}", groups);
        let group = groups.iter().map(|x| x.chars().collect::<HashSet<char>>()).collect::<Vec<HashSet<char>>>();
        let mut all_chars: HashSet<_> = HashSet::new();
        for member in group.iter() {
            all_chars = all_chars.union(&member).map(|&x| x).collect();
        }
        let mut sec_chars: HashSet<_> = all_chars.clone();
        for member in group.iter() {
            sec_chars = sec_chars.intersection(&member).map(|&x| x).collect();
        }
        vector.push(sec_chars.len() as i32);
    }
    vector.clone()
}

fn grouping(lines: Vec<&str>) -> Vec<i32> {
    let mut groups: Vec<i32> = Vec::new();
    let mut set: HashSet<char> = HashSet::new();
    for group in lines.split(|&x| x == "") {
        for line in group {
            for char in line.chars() {
                set.insert(char);
            }
        }
        groups.push(set.len() as i32);
        set.clear();
    }
    groups.clone()
}
