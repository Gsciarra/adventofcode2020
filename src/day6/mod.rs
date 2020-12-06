use crate::utils::{get_input, print_solution};
use std::collections::HashSet;

pub fn solution1() {
    let input = get_input::<String>("input-day6.txt").unwrap();
    let mut group_answer: HashSet<char> = HashSet::new();
    let mut tot = 0;
    input.lines().for_each(|line| {
        if line.is_empty() {
            tot = tot + group_answer.len();
            group_answer.clear();
        } else {
            group_answer = group_answer
                .union(&line.chars().collect())
                .map(|&c| c)
                .collect();
        }
    });

    tot = tot + group_answer.len();

    print_solution(6, 1, tot);
}

pub fn solution2() {
    let input = get_input::<String>("input-day6.txt").unwrap();
    let mut group_answer: HashSet<char> = HashSet::new();
    let mut tot = 0;

    let mut just_cleared = true;
    input.lines().for_each(|line| {
        if line.is_empty() {
            tot = tot + group_answer.len();
            group_answer.clear();
            just_cleared = true;
        } else if just_cleared {
            just_cleared = false;
            group_answer = line.chars().collect()
        } else {
            just_cleared = false;
            group_answer = group_answer
                .intersection(&line.chars().collect())
                .map(|&c| c)
                .collect();
        }
    });

    tot = tot + group_answer.len();

    print_solution(6, 2, tot);
}
