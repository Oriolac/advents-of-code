use std::env;
use std::fs;


struct Password {
    from: usize,
    to: usize,
    letter: char,
    word: String,
}


trait Validation1 {
    fn is_valid1(&self) -> bool;
}

trait Validation2 {
    fn is_valid2(&self) -> bool;
}

impl Validation1 for Password {
    fn is_valid1(&self) -> bool {
        let count = self.word.chars().filter(|x| x == &self.letter).count();
        return self.from <= count && count <= self.to;
    }
}

impl Validation2 for Password {
    fn is_valid2(&self) -> bool {
        let first = self.word.chars().nth(self.from - 1).unwrap() == self.letter;
        let second = self.word.chars().nth(self.to - 1).unwrap() == self.letter;
        return first && !second || !first && second;
    }
}

fn parse(words: Vec<&str>) -> Password {
    let range: Vec<usize> = words[0].split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .rev().collect();
    return Password{from: range[1], to: range[0], letter: words[1].chars().next().unwrap(), word: String::from(words[2])};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])
        .expect("File not found");
    let passwords = contents.split("\n").map(|x| x.split_whitespace()
        .collect::<Vec<&str>>())
        .map(|x| parse(x)).collect::<Vec<Password>>();
    println!("{}", passwords.iter().filter(|x| x.is_valid1()).count());
    println!("{}", passwords.iter().filter(|x| x.is_valid2()).count());

}
