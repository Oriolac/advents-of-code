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

struct Player {
    x: usize,
    y: usize,
    trees: usize,
}

impl Player {
    fn move_to(&mut self, worldmap: Vec<Vec<Tile>>) {
        let module = worldmap[0].len();
        self.x = (self.x + 3) % module;
        self.y += 1;
        self.trees += match worldmap[self.y][self.x] {
            Tree() => 1,
            Nothing() => 0,
        };
    }

    fn has_finished(&mut self, size: usize) -> bool {
        return self.y != size - 1;
    }
}

fn trees_part_1(worldmap: Vec<Vec<Tile>>) -> usize {
    let mut player = Player { x: 0, y: 0, trees: 0 };
    while player.has_finished(worldmap.len()) {
        player.move_to(worldmap.clone());
    }
    player.trees
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: String = fs::read_to_string(&args[1])
        .expect("Error reading the file");
    let worldmap: Vec<Vec<Tile>> = file
        .split('\n')
        .map(|x| parse(String::from(x)))
        .collect();
    println!("{}", trees_part_1(worldmap))
}
