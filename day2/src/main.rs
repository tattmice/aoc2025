use std::{fs};

const FILE_PATH: &'static str = "src/in.txt";

struct IDPair {
    first: i64,
    last: i64,
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect("Unable to read the input file")
}

fn part1(ids: &Vec<IDPair>) -> i64 {
    return solve(ids, is_invalid_p1);
}

fn part2(ids: &Vec<IDPair>) -> i64 {
    return solve(ids, is_invalid_p2);
}

fn solve(ids: &Vec<IDPair>, is_invalid: fn(&String) -> bool) -> i64 {
    let mut total = 0;

    for id in ids {
        total += process_pair(id, is_invalid);
    }

    return total;
}

fn process_pair(pair: &IDPair, is_invalid: fn(&String) -> bool) -> i64 {
    let mut total: i64 = 0;
    let mut id: String = pair.first.to_string();
    let l: String = (pair.last + 1).to_string();
    while id != l {
        let id_int: i64 = id.parse().unwrap();
        if is_invalid(&id) {
            // println!("invalid id found: {}", id);
            total += id_int;
        }
        id = (id_int + 1).to_string();
    }

    return total;
}

fn is_invalid_p1(id: &String) -> bool {
    if id.len() % 2 == 1 {
        return false;
    }

    let midpoint = id.len() / 2;
    return id[0..midpoint] == id[midpoint..id.len()];
}

fn is_invalid_p2(id: &String) -> bool {
    for i in 1..((id.len() / 2) + 1) {
        if id.len() % i != 0 {
            continue;
        }

        let splits: Vec<char> = id.chars().collect();
        let mut chunks = splits.chunks(i);

        let first_chunk = chunks.next().unwrap();
        let mut chunks_equals: bool = true;
        for c in chunks {
            if c != first_chunk {
                chunks_equals = false;
                break
            }
        }

        if chunks_equals {
            return true;
        }
    }

    return false;
}

fn main() {
    let contents = read_file(FILE_PATH);

    let transform = |s: &str| -> IDPair {
        let [f, e] = s.split('-')
        .map(|t: &str| t.parse().expect("Failed to parse id"))
        .collect::<Vec<i64>>()
        .try_into()
        .unwrap();

        return IDPair{
            first: f,
            last: e,
        };
    };

    let ids: Vec<IDPair> = contents.split(',').map(transform).collect::<Vec<IDPair>>();

    let p1 = part1(&ids);
    println!("Part 1: {}", p1);

    let p2 = part2(&ids);
    println!("Part 2: {}", p2);
}
