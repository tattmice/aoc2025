use std::fs;

const FILE_PATH: &'static str = "src/in.txt";
const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1)
];

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect("Unable to read the input file")
}

fn part1(grid: &Vec<String>) -> i64 {
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i].chars().nth(j).unwrap() != '@' {
                continue;
            }

            if count_adjacent(grid, i, j) < 4 {
                count += 1;
            }
        }
    }
    return count;
}

fn count_adjacent(grid: &Vec<String>, i: usize, j: usize) -> i64 {
    let mut count = 0;

    for dir in DIRS {
        if dir.0 > 0 && i + 1 == grid.len() {
            continue;
        }
        if dir.1 > 0 && j + 1 == grid[0].len() {
            continue;
        }

        let Some(newi) = i.checked_add_signed(dir.0) else { continue; };
        let Some(newj) =  j.checked_add_signed(dir.1) else { continue; };

        // println!("{} {} ({}, {}) {} {}", i, j, dir.0, dir.1,  newi, newj);
        if grid[newi].chars().nth(newj).unwrap() == '@' {
            count += 1;
        }
    }

    return count;
}

fn part2(grid: &mut Vec<String>) -> i64 {
    let mut count = 0;
    let mut removed = true;

    while removed {
        removed = false;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i].chars().nth(j).unwrap() != '@' {
                    continue;
                }

                if count_adjacent(grid, i, j) < 4 {
                    removed = true;
                    grid[i].replace_range(j..j+1, ".");
                    count += 1;
                }
            }
        }
    }
    
    return count;
}

fn main() {
    let contents = read_file(FILE_PATH);
    let mut grid: Vec<String> = contents.split('\n').map(String::from).collect();

    let p1 = part1(&grid);
    println!("Part 1: {}", p1);

    let p2 = part2(&mut grid);
    println!("Part 2: {}", p2);
}
