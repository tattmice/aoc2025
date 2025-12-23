use std::fs;

const FILE_PATH: &'static str = "src/in.txt";

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect("Unable to read the input file")
}

fn part1(banks: &Vec<&str>) -> i64 {
    return total_joltage(banks, 2);
}

fn part2(banks: &Vec<&str>) -> i64 {
    return total_joltage(banks, 12);
}

fn total_joltage(banks: &Vec<&str>, n: usize) -> i64 {
        let mut total = 0;

    for bank in banks {
        total += max_joltage(bank, n);
    }

    return total
}

fn max_joltage(bank: &str, n: usize) -> i64 {
    let mut joltage = 0;
    let mut remaining_digits  = n;
    let mut current_bank = bank;

    while remaining_digits > 0 && current_bank.len() > remaining_digits {
        let (best_battery, index_found) = best_digit(current_bank, remaining_digits);
        // println!("  bank={}, bat={}, i={}", current_bank, best_battery, index_found);
        joltage *= 10;
        joltage += best_battery;
        remaining_digits -= 1;
        current_bank = &current_bank[index_found+1..]
    }

    if remaining_digits > 0 {
        joltage *= i64::pow(10, remaining_digits as u32);
        joltage += current_bank.parse::<i64>().unwrap();
    }

    // println!("joltage is {}", joltage);
    return joltage;
}

fn best_digit(bank: &str, remaining_digits: usize) -> (i64, usize) {
    let mut j = 0;
    let mut idx = 0;

    for (i, battery) in bank.chars().take(bank.len() - remaining_digits + 1).enumerate() {
        let battery_value = i64::from(battery.to_digit(10).unwrap());
        if j < battery_value {
            j = battery_value;
            idx = i;
        }
    }

    return (j, idx);
}

fn main() {
    let contents = read_file(FILE_PATH);
    let banks: Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&banks);
    println!("Part 1: {}", p1);

    let p2 = part2(&banks);
    println!("Part 2: {}", p2);
}
