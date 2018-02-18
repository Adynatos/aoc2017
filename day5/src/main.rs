
fn part1(input: &str) -> u32{
    let mut count = 0;

    let mut array: Vec<i32> = input.split_whitespace()
                               .flat_map(|x| x.parse())
                               .collect();
    let length: i32 = array.len() as i32;

    let mut i: i32 = 0;
    while i < length {
        let cmd = array[i as usize];
        array[i as usize] += 1;
        i += cmd;
        count += 1;
    }

    count
}

fn part2(input: &str) -> u32 {
    let mut count = 0;

    let mut array: Vec<i32> = input.split_whitespace()
                               .flat_map(|x| x.parse())
                               .collect();
    let length: i32 = array.len() as i32;

    let mut i: i32 = 0;
    while i < length {
        let cmd = array[i as usize];
        if cmd > 2 {
            array[i as usize] -= 1;
        } else {
            array[i as usize] += 1;
        }
        i += cmd;
        count += 1;
    }

    count
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
