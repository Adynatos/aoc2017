use std::collections::HashSet;

fn part1(input: &str) -> u32{
    let mut sum = 0;

    for line in input.lines() {
        let passphrases = line.split_whitespace();
        let mut set = HashSet::new();
        let mut size = 0;
        for pass in passphrases {
            size += 1;
            set.insert(pass);
        }
        if set.len() == size {
            sum += 1;
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let passphrases = line.split_whitespace();
        let mut set = HashSet::new();
        let mut size = 0;
        for pass in passphrases {
            let mut chars: Vec<char> = pass.chars().collect();
            chars.sort();
            size += 1;
            set.insert(chars);
        }
        if set.len() == size {
            sum += 1;
        }
    }
    sum
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
