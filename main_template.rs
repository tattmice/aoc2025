use std::fs;

const FILE_PATH: &'static str = "src/in.txt";

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect("Unable to read the input file")
}

fn part1(operations: &Vec<&str>) -> i64 {
   return 0;
}

fn part2(operations: &Vec<&str>) -> i64 {
    return 0;
}

fn main() {
    let contents = read_file(FILE_PATH);
    let operations: Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&operations);
    println!("Part 1: {}", p1);

    let p2 = part2(&operations);
    println!("Part 2: {}", p2);
}
