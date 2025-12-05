use std::fs;

const FILE_PATH: &'static str = "src/in.txt";

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect("Unable to read the input file")
}

fn part1(operations: &Vec<&str>) -> i64 {
    let mut pos: i64 = 50;
    let mut count: i64 = 0;

    let f = |o: &&str| {
        let dir = o.chars().next().expect("Unable to parse value from file");
        let mut amount: i64 = o[1..].parse().expect("Unable to parse value from file");

        if dir == 'L' {
            amount *= -1;
        }

        pos = (pos + amount) % 100;
        if pos < 0 {
            pos += 100;
        }

        // println!("Moving {}! count={}", o, count);

        if pos == 0 {
            count += 1;
        }
    };

    operations.iter().for_each(f);
    return count;
}

fn part2(operations: &Vec<&str>) -> i64 {
    let mut pos: i64 = 50;
    let mut count: i64 = 0;

    let f = |o: &&str| {
        let dir = o.chars().next().expect("Unable to parse value from file");
        let mut amount: i64 = o[1..].parse().expect("Unable to parse value from file");

        if dir == 'L' {
            amount = -amount;
        }

        let mut rotations = (pos + amount) / 100;

        if rotations < 0 {
            rotations *= -1;
        }
        if pos + amount <= 0 && pos != 0 {
            rotations += 1;
        }

        count += rotations;

        pos = (pos + amount) % 100;
        if pos < 0 {
            pos += 100;
        }

        // println!("Moving {} to {} with {} rotations! count={}", o, pos, rotations, count);
    };

    operations.iter().for_each(f);
    return count;
}

fn main() {
    let contents = read_file(FILE_PATH);
    // println!("contents: {}", contents);

    let operations: Vec<&str> = contents.split('\n').collect();
    // println!("operations: {:#?}", operations);

    let p1 = part1(&operations);
    println!("Part 1: {}", p1);

    let p2 = part2(&operations);
    println!("Part 2: {}", p2);
}
