use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
//use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, isize, usize};

const DAY: &str = "9";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "2333133121414131402";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);
    println!("TEST PASSED");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    //println!("\n=== Part 2 ===");
    //assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);
    //println!("TEST PASSED");
    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part2(input_file)?);
    //println!("Result = {}", result);
    //endregion

    Ok(())
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut bob = reader
        .lines()
        .next()
        .expect("bob")?
        .chars()
        .collect::<Vec<char>>();
    if bob.len() % 2 != 0 {
        bob.push('0');
    }
    //println!("{bob:?}");
    let mut iterator = bob.chunks(2).enumerate();

    let mut result: Vec<BlockOrFree> = Vec::new();

    while let Some((index, [data, size])) = iterator.next() {
        dbg!(index);
        let data = data.to_digit(10).expect("invalid digit");
        let free_size = size.to_digit(10).expect("invalid Digit");

        (0..data).for_each(|_| {
            result.push(BlockOrFree::Block(index));
        });
        (0..free_size).for_each(|_| {
            result.push(BlockOrFree::Free);
        });
    }

    let mut upper_limit = result.len() - 1;
    let mut lower_limit = 0;

    while upper_limit > lower_limit {
        if result[upper_limit] == BlockOrFree::Free {
            upper_limit -= 1;
            continue;
        }

        while result[lower_limit] != BlockOrFree::Free {
            lower_limit += 1;
        }

        result[lower_limit] = result[upper_limit];
        result[upper_limit] = BlockOrFree::Free;

        upper_limit -= 1;
        lower_limit += 1;
    }

    //println!("{result:?}");

    let answer = result
        .iter()
        .enumerate()
        .fold(0, |acc, (index, block)| match block {
            BlockOrFree::Free => acc,
            BlockOrFree::Block(id) => acc + (index * id),
        });

    Ok(answer)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    todo!()
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum BlockOrFree {
    Block(usize),
    Free,
}
