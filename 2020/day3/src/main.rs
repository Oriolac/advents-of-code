use std::{env, fs};
use reduce::Reduce;

#[derive(Clone, Copy, Debug)]
enum Tile { Snow, Tree }

fn parse(line: String) -> Vec<Tile> {
    return line.chars()
        .map(|x| {
            return match x {
                '.' => Some(Tile::Snow),
                '#' => Some(Tile::Tree),
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
    fn move_to(&mut self, worldmap: Vec<Vec<Tile>>, x: usize, y: usize) {
        let module = worldmap[0].len();
        self.x = (self.x + x) % module;
        self.y += y;
        self.trees += match worldmap[self.y][self.x] {
            Tile::Tree => 1,
            Tile::Snow => 0,
        };
    }

    fn has_finished(&mut self, size: usize) -> bool {
        return self.y != size - 1;
    }
}

fn tree_count(worldmap: Vec<Vec<Tile>>, x: usize, y: usize) -> usize {
    let mut player = Player { x: 0, y: 0, trees: 0 };
    while player.has_finished(worldmap.len()) {
        player.move_to(worldmap.clone(), x, y);
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
    println!("{}", tree_count(worldmap.clone(), 3, 1));
    let moves: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!("{:?}",
             moves.into_iter()
                 .map(|x| tree_count(worldmap.clone(), x.0, x.1))
                 .reduce(|x, y| x * y).unwrap());
}
