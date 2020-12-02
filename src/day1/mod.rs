pub fn find_combination_to_obtain_n(numbers: &Vec<usize>, missing_n_in_combination: usize, n: usize) -> Option<Vec<usize>> {
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