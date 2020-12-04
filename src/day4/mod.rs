use std::fs::File;
use std::io::Read;
use regex::Regex;
use crate::utils::get_input;

pub fn solution() {
    let mut input: String = get_input("input-day4.txt").unwrap();

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut remain_to_check = required_fields.clone();

    let mut total_valid = 0;

    for line in input.lines() {
        if line.len() > 0 {
            let fields: Vec<&str> = line
                .split_whitespace()
                .filter_map(|kv| {
                    Some(kv.split(":").collect::<Vec<&str>>()[0])
                })
                .collect();
            remain_to_check = remain_to_check.iter().filter(|k| !fields.contains(k)).map(|k| *k).collect();
        } else {
            if remain_to_check.len() == 0 {
                total_valid = total_valid + 1;
            }
            remain_to_check = required_fields.clone();
        }
    }

    if remain_to_check.len() == 0 {
        total_valid = total_valid + 1;
    }

    println!("Day 4 - part 1: {}", total_valid);
}

pub fn solution2() {
    let mut input = String::new();
    let _read_result = File::open("input-day4.txt")
        .unwrap()
        .read_to_string(&mut input);

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut remain_to_check = required_fields.clone();

    let mut total_valid = 0;

    for line in input.lines() {
        if line.len() > 0 {
            let valid_fields: Vec<&str> = line
                .split_whitespace()
                .filter_map(|kv| {
                    let key_value = kv.split(":").collect::<Vec<&str>>();
                    if validation(&key_value) {
                        Some(key_value[0])
                    } else {
                        None
                    }
                })
                .collect();
            remain_to_check = remain_to_check.iter().filter(|k| !valid_fields.contains(k)).map(|k| *k).collect();
        } else {
            if remain_to_check.len() == 0 {
                total_valid = total_valid + 1;
            }
            remain_to_check = required_fields.clone();
        }
    }

    if remain_to_check.len() == 0 {
        total_valid = total_valid + 1;
    }

    println!("Day 4 - part 2: {}", total_valid);
}

fn validation(k_v_field: &Vec<&str>) -> bool {
    let key = k_v_field[0];
    let value = k_v_field[1];

    match key {
        "byr" => {
            value.len() == 4 && match value.parse::<usize>() {
                Ok(v) => { v >= 1920 && v <= 2002 }
                Err(_) => { false }
            }
        }
        "iyr" => {
            value.len() == 4 && match value.parse::<usize>() {
                Ok(v) => { v >= 2010 && v <= 2020 }
                Err(_) => { false }
            }
        }
        "eyr" => {
            value.len() == 4 && match value.parse::<usize>() {
                Ok(v) => { v >= 2020 && v <= 2030 }
                Err(_) => { false }
            }
        }
        "hgt" => {
            let len = value.len();
            let num_part = &value[..len - 2];
            let unit_part = &value[len - 2..];

            match num_part.parse::<i32>() {
                Ok(v) => {
                    match unit_part {
                        "cm" => v >= 150 && v <= 193,
                        "in" => v >= 59 && v <= 76,
                        _ => false
                    }
                }
                Err(_) => { false }
            }
        }
        "hcl" => is_color(value),
        "ecl" => is_eye_color(value),
        "pid" => {
            value.len() == 9 && match value.parse::<usize>() {
                Ok(_) => { true }
                Err(_) => { false }
            }
        }

        _ => true
    }
}

fn is_color(s: &str) -> bool {
    Regex::new("^#[a-f0-9]{6}$").unwrap().is_match(s)
}

fn is_eye_color(s: &str) -> bool {
    let accepted = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    accepted.contains(&s)
}