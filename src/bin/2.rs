use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "2";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(PartialEq, Eq)]
enum Order {
    Ascending,
    Descending,
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut answer = 0;

    for line in reader.lines() {
        let numbers: Vec<usize> = line?
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        //println!("{numbers:?}");
        let peekable_iteration = numbers.iter().peekable();
        if is_safe(peekable_iteration) {
            answer += 1;
            continue;
        }
    }

    Ok(answer)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut answer = 0;

    for line in reader.lines() {
        let numbers: Vec<usize> = line?
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        //println!("{numbers:?}");
        let peekable_iteration = numbers.iter().peekable();
        if is_safe(peekable_iteration) {
            answer += 1;
            continue;
        }

        // O lord forgive me for this
        for index in 0..numbers.len() {
            let mut numbers_without_one_number = numbers.clone();
            numbers_without_one_number.remove(index);
            let peekable_iteration = numbers_without_one_number.iter().peekable();
            if is_safe(peekable_iteration) {
                answer += 1;
                break;
            }
        }
    }

    Ok(answer)
}

fn is_safe(mut peekable_iteration: std::iter::Peekable<std::slice::Iter<'_, usize>>) -> bool {
    let mut current_order: Option<Order> = None;
    while let Some(number) = peekable_iteration.next() {
        let Some(next_number) = peekable_iteration.peek() else {
            return true;
        };

        match number.cmp(next_number) {
            Ordering::Less => {
                if let Some(order) = &current_order {
                    if *order == Order::Descending {
                        break;
                    }
                } else {
                    current_order = Some(Order::Ascending);
                }
            }
            Ordering::Greater => {
                if let Some(order) = &current_order {
                    if *order == Order::Ascending {
                        break;
                    }
                } else {
                    current_order = Some(Order::Descending);
                }
            }
            Ordering::Equal => break,
        }

        if number.abs_diff(**next_number) > 3 {
            break;
        }
    }
    false
}
