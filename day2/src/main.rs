fn part1(input: &str) -> u32{
    let mut sum = 0;

    for line in input.lines() {
        let line = line.split_whitespace();
        let array: Vec<u32> = line.flat_map(|e| e.parse()).collect();
        let max = array.iter().max().unwrap();
        let min= array.iter().min().unwrap();

        sum += max - min;
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let line = line.split_whitespace();
        let array: Vec<u32> = line.flat_map(|e| e.parse()).collect();

        for i in 0..array.len() {
            for j in 0..array.len() {
                if array[i] % array[j] == 0 && i != j {
                    sum += array[i] / array[j];
                }
            }
        }
    }

    sum
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
