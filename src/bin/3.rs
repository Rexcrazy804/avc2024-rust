use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "3";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut answer = 0;

    for line in reader.lines() {
        let mut buffer = String::new();
        let mut buffer_active = false;

        for char in line?.chars() {
            match char {
                'm' => {
                    if !buffer_active {
                        buffer_active = true;
                    } else {
                        buffer.clear();
                    }
                }
                ')' => {
                    buffer.push(')');
                    buffer_active = false;
                    if let Some((num1, num2)) = process_mull_buffer(&buffer) {
                        answer += num1 * num2;
                    }
                    buffer.clear();
                }
                _ => {}
            }

            if buffer_active {
                buffer.push(char);
            }
        }
    }

    Ok(answer)
}

fn process_mull_buffer(buffer: &String) -> Option<(usize, usize)> {
    use std::result::Result::Ok;
    let mut tokens = buffer.split(['(', ',', ')']).take(3);
    let mul_token = tokens.next()?;
    let number_1 = tokens.next()?;
    let number_2 = tokens.next()?;

    if mul_token != "mul" {
        return None;
    }

    let Ok(number_1) = number_1.parse::<usize>() else {
        return None;
    };

    let Ok(number_2) = number_2.parse::<usize>() else {
        return None;
    };

    Some((number_1, number_2))
}

enum ToDoOrNotToDo {
    Do,
    Dontdo,
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut answer = 0;

    let mut do_multiplication = true;
    for line in reader.lines() {
        let mut mull_buffer = String::new();
        let mut mull_buffer_active = false;

        let mut do_or_dont_buffer = String::new();
        let mut do_or_dont_buffer_active = false;

        for char in line?.chars() {
            match char {
                'm' => {
                    if !mull_buffer_active {
                        mull_buffer_active = do_multiplication;
                    } else {
                        mull_buffer.clear();
                    }
                }

                'd' => {
                    if !do_or_dont_buffer_active {
                        do_or_dont_buffer_active = true;
                    } else {
                        do_or_dont_buffer.clear();
                    }
                }
                ')' => {
                    if do_or_dont_buffer_active {
                        do_or_dont_buffer.push(')');
                        do_or_dont_buffer_active = false;

                        if let Some(action) = process_do_or_dont_buffer(&do_or_dont_buffer) {
                            match action {
                                ToDoOrNotToDo::Do => do_multiplication = true,
                                ToDoOrNotToDo::Dontdo => do_multiplication = false,
                            }
                        }
                        do_or_dont_buffer.clear();
                    }
                    if mull_buffer_active {
                        mull_buffer.push(')');
                        mull_buffer_active = false;
                        if let Some((num1, num2)) = process_mull_buffer(&mull_buffer) {
                            if do_multiplication {
                                answer += num1 * num2;
                            }
                        }
                        mull_buffer.clear();
                    }
                }
                _ => {}
            }

            if mull_buffer_active {
                mull_buffer.push(char);
            }

            if do_or_dont_buffer_active {
                do_or_dont_buffer.push(char);
            }
        }
    }

    Ok(answer)
}

fn process_do_or_dont_buffer(do_or_dont_buffer: &str) -> Option<ToDoOrNotToDo> {
    let mut tokens = do_or_dont_buffer.split('(').take(2);
    let do_or_dont = tokens.next()?;
    if tokens.next()? != ")" {
        return None;
    }

    if do_or_dont == "do" {
        return Some(ToDoOrNotToDo::Do);
    }

    if do_or_dont == "don't" {
        return Some(ToDoOrNotToDo::Dontdo);
    }

    None
}
