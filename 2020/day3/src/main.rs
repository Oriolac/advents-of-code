use std::{env, fs};
use crate::Tile::{Nothing, Tree};

#[derive(Clone, Copy, Debug)]
enum Tile { Nothing(), Tree() }

fn parse(line: String) -> Vec<Tile> {
    return line.chars()
        .map(|x| {
            return match x {
                '.' => Some(Nothing()),
                '#' => Some(Tree()),
                _ => None,
            }.unwrap();
        })
        .collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: String = fs::read_to_string(&args[1])
        .expect("Error reading the file");
    let worldmap: Vec<Vec<Tile>> = file
        .split('\n')
        .map(|x| parse(String::from(x)))
        .collect();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut trees: usize = 0;
    let module = worldmap[0].len();
    while y != worldmap.len() - 1 {
        x = (x + 3) % module;
        y += 1;
        trees += match worldmap[y][x] {
            Tree() => 1,
            Nothing() => 0,
        };
    }
    println!("{}", trees)
}
