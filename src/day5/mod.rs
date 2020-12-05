use crate::utils::{get_input};

pub fn solution1() {
    let input = get_input::<String>("input-day5.txt").unwrap();
    let mut max = 0f64;

    input.lines().for_each(|line| {
        let row = get_row(&line[..7]);
        let column = get_column(&line[7..10]);
        let id = row * 8f64 + column;
        if id > max {
            max = id
        }
    });

    println!("{:?}", max);
}

pub fn solution2() {
    let input = get_input::<String>("input-day5.txt").unwrap();
    let mut ids: Vec<usize> = vec![];

    input.lines().for_each(|line| {
        let row = get_row(&line[..7]);
        let column = get_column(&line[7..10]);
        let id = row * 8f64 + column;
        ids.push(id as usize);
    });

    ids.sort();

    let mut my_id = ids.last().unwrap() + 1;

    for (i, id) in ids.iter().skip(1).enumerate() {
        if id - ids[i] != 1 {
            my_id = id - 1
        }
    }

    println!("{:?}", my_id);
}

fn get_row(indications: &str) -> f64 {
    let mut range = (0f64, 127f64);

    for indication in indications[..6].chars() {
        let m: f64 = ((range.0 + range.1) / 2f64);

        if indication == 'F' {
            range.1 = m.floor()
        } else {
            range.0 = m.ceil()
        }
    }

    if range.1 - range.0 != 1f64 {
        panic!(format!("RANGE INVALID: {:?}", range))
    }

    if indications.chars().nth(6).unwrap() == 'F' {
        range.0
    } else {
        range.1
    }
}

fn get_column(indications: &str) -> f64 {
    let mut range = (0f64, 7f64);

    for indication in indications[..2].chars() {
        let m: f64 = ((range.0 + range.1) / 2f64);

        if indication == 'L' {
            range.1 = m.floor()
        } else {
            range.0 = m.ceil()
        }
    }

    if range.1 - range.0 != 1f64 {
        panic!(format!("RANGE INVALID: {:?}", range))
    }

    if indications.chars().nth(2).unwrap() == 'L' {
        range.0
    } else {
        range.1
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::{get_row, get_column};

    #[test]
    fn it_works() {
        assert_eq!(get_row(&"BFFFBBFRRR"[..7]) * 8f64 + get_column(&"BFFFBBFRRR"[7..10]), 567f64);
        assert_eq!(get_row(&"FFFBBBFRRR"[..7]) * 8f64 + get_column(&"FFFBBBFRRR"[7..10]), 119f64);
        assert_eq!(get_row(&"BBFFBBFRLL"[..7]) * 8f64 + get_column(&"BBFFBBFRLL"[7..10]), 820f64);
        assert_eq!(get_row(&"FFFFFFFLLL"[..7]) * 8f64 + get_column(&"FFFFFFFLLL"[7..10]), 0f64);
        assert_eq!(get_row(&"BBBBBBBRRR"[..7]) * 8f64 + get_column(&"BBBBBBBRRR"[7..10]), 127f64 * 8f64 + 7f64);
    }
}