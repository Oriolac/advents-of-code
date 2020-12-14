use std::{env, fs};
use std::str::Split;
use reduce::Reduce;
use std::cmp::max;
use itertools::Tuples;


struct Seat {
    row: i32,
    column: i32,
}

impl Seat {
    fn get_id(self) -> i32 { self.row * 8 + self.column }


    fn from_position(row_str: &str, col_str: &str) -> Seat {
        let row = get_position(row_str.chars().collect::<Vec<char>>(), 0, 127, 'F', 'B');
        let column = get_position(col_str.chars().collect::<Vec<char>>(), 0, 7, 'L', 'R');
        Seat{row, column}
    }
}

fn get_position(chars: Vec<char>, immut_from: i32, immut_to: i32, lower: char, upper: char) -> i32 {
    let mut from = immut_from.clone();
    let mut to = immut_to.clone();
    for &char in chars.iter() {
        if char.eq(&lower) {
            to = (to - from) / 2 + from;
        } else if char.eq(&upper) {
            from += (to - from) / 2 + 1;
        } else { panic!("NOT UPPER NEITHER LOWER IN {}", char) }
    }
    from
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file= fs::read_to_string(&args[1]).unwrap();
    let lines = file.split("\n");
    println!("{}", to_seat("FBFBBFFRLR").get_id());
    let seats = lines.map(|x| to_seat_id(x)).collect::<Vec<i32>>();
    println!("{}", seats.iter().reduce(max).unwrap());
}

fn to_seat_id(line: &str) -> i32 {
    to_seat(line).get_id()
}

fn to_seat(line: &str) -> Seat {
    let position = line.split_at(7);
    return Seat::from_position(position.0, position.1);
}


