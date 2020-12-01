use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _read_result = File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input);

    let numbers: Vec<usize> = input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let combination = find_combination_to_obtain_n(&numbers, 3, 2020);

    println!("input: {:?}", numbers);
    println!("combination: {:?}", combination);
    println!("result: {:?}", combination.unwrap().iter().fold(1, |acc, x| acc * x));
}


fn find_combination_to_obtain_n(numbers: &Vec<usize>, missing_n_in_combination: usize, n: usize) -> Option<Vec<usize>> {
    if numbers.len() < missing_n_in_combination {
        None
    } else if numbers.len() == missing_n_in_combination {
        if numbers.iter().sum::<usize>() == n {
            Some(numbers.clone())
        } else {
            None
        }
    } else {
        if missing_n_in_combination == 1 {
            if numbers.contains(&n) { Some(vec![n]) } else { None }
        } else {
            for (i, &x) in numbers.iter().enumerate() {
                if n > x {
                    let current_n = n - x;
                    return match find_combination_to_obtain_n(&numbers[i + 1..].to_vec(), missing_n_in_combination - 1, current_n) {
                        Some(mut results) => {
                            results.push(x);
                            Some(results)
                        }
                        None => continue,
                    };
                }
            }
            None
        }
    }
}