use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "1";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[allow(dead_code)]
fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut first_numbers: Vec<usize> = Vec::new();
    let mut second_numbers: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let (first, second) = line.split_once(' ').unwrap();
        let second = second.trim();

        first_numbers.push(first.parse()?);
        second_numbers.push(second.parse()?);

        //println!("{line}: {first},{second}");
    }

    //println!("{first_numbers:?} {second_numbers:?}");
    first_numbers.sort();
    second_numbers.sort();

    let answer = first_numbers
        .iter()
        .zip(second_numbers.iter())
        .fold(0, |acc, (first, second)| acc + first.abs_diff(*second));

    Ok(answer)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut first_numbers: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let (first, second) = line.split_once(' ').unwrap();
        let (first, second): (usize, usize) = (first.parse()?, second.trim().parse()?);

        map.entry(second)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        first_numbers.push(first);
    }

    let answer = first_numbers.iter().fold(0, |answer, number| {
        answer + (number * map.get(number).unwrap_or(&0))
    });
    Ok(answer)
}
