use std::cmp;
use std::collections::HashMap;

fn part1(input: i32) -> i32{
    let mut x: i32 = 0;
    let mut y: i32  = 0;
    let mut dx = 1;
    let mut dy = 0;

    for _ in 1..input {
        x += dx;
        y += dy;

        let dist = cmp::max(x.abs(), y.abs());

        if x == dist {
            dx = 0;
            dy = 1;
        }

        if y == dist {
            dx = -1;
            dy = 0;
        }

        if x == -dist {
            dx = 0;
            dy = -1;
        }

        if y == -dist {
            dx = 1;
            dy = 0;
        }
    }

    x.abs() + y.abs()
}

fn part2(input: i32) -> i32{
    let mut x: i32 = 0;
    let mut y: i32  = 0;
    let mut dx = 1;
    let mut dy = 0;

    let mut set = HashMap::new();
    set.insert((0, 0), 1);

    for _ in 1..input {
        x += dx;
        y += dy;

        let mut value = 0;
        if let Some(neigh) = set.get(&(x - 1, y)) {
            value += neigh;
        }
        if let Some(neigh) = set.get(&(x - 1, y - 1)) {
            value += neigh;
        }
        if let Some(neigh) = set.get(&(x - 1, y + 1)) {
            value += neigh;
        }
        if let Some(neigh) = set.get(&(x + 1, y - 1)) {
            value += neigh;
        }
        if let Some(neigh) = set.get(&(x + 1, y)) {
            value += neigh;
        }
        if let Some(neigh) = set.get(&(x + 1, y + 1)) {
            value += neigh;
        }
        if let Some(neigh) = set.get(&(x, y + 1)) {
            value += neigh;
        }
        if let Some(neigh) = set.get(&(x, y - 1)) {
            value += neigh;
        }

        set.insert((x, y), value);

        if value > input {
            return value;
        }

        let dist = cmp::max(x.abs(), y.abs());

        if x == dist {
            dx = 0;
            dy = 1;
        }

        if y == dist {
            dx = -1;
            dy = 0;
        }

        if x == -dist {
            dx = 0;
            dy = -1;
        }

        if y == -dist {
            dx = 1;
            dy = 0;
        }
    }

    return 0;
}

fn main() {
    let input = 277678;

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
