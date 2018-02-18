use std::collections::HashSet;

fn part1(input: &str) -> u32{
    let mut count = 0;

    let mut array: Vec<i32> = input.split_whitespace()
                               .flat_map(|x| x.parse())
                               .collect();
    let mut set = HashSet::new();
    loop {
        let mut max: i32 = *array.iter().max().unwrap();
        let mut idx = array.iter().position(|e| e == &max).unwrap();

        array[idx] = 0;

        let size = array.len();
        idx += 1;
        while max > 0 {
            array[idx % size] += 1;
            max -= 1;
            idx += 1;
        }

        count += 1;
        if !set.insert(array.clone()) {
            return count;
        }
    }
}

fn part2(input: &str) -> usize{
    let mut array: Vec<i32> = input.split_whitespace()
                               .flat_map(|x| x.parse())
                               .collect();
    let mut set = Vec::new();
    loop {
        let mut max: i32 = *array.iter().max().unwrap();
        let mut idx = array.iter().position(|e| e == &max).unwrap();

        array[idx] = 0;

        let size = array.len();
        idx += 1;
        while max > 0 {
            array[idx % size] += 1;
            max -= 1;
            idx += 1;
        }

        if let Some(idx) = set.iter().position(|e| *e == array) {
            println!("{}, {}", set.len(), idx);
            return (set.len() - (idx ));
        }
        set.push(array.clone());
    }
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
