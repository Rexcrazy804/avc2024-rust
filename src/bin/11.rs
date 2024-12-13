use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, isize, usize};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "125 17";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);
    println!("TEST PASSED");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);
    //println!("TEST PASSED");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let lone_line = reader.lines().next().unwrap()?;
    let mut seed = Seed::parse(&lone_line);

    for _ in 0..25 {
        seed.blink();
    }

    //dbg!(&seed.line);
    let answer = seed.line.values().fold(0, |acc, count| acc + count);
    Ok(answer)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let lone_line = reader.lines().next().unwrap()?;
    let mut seed = Seed::parse(&lone_line);

    for _ in 0..75 {
        seed.blink();
    }

    let answer = seed.line.values().fold(0, |acc, count| acc + count);
    Ok(answer)
}

struct Seed {
    line: HashMap<usize, usize>,
}

impl Seed {
    fn parse(str: &str) -> Self {
        let line = str
            .split_whitespace()
            .map(|x| x.parse::<usize>().expect("Invalid digit"))
            .collect_vec();

        let mut map = HashMap::new();

        for number in line {
            map.entry(number).and_modify(|v| *v += 1).or_insert(1);
        }

        Self { line: map }
    }

    fn blink(&mut self) {
        for (stone, stone_count) in self.line.clone().iter() {
            let stone = *stone;

            self.line.entry(stone).and_modify(|v| *v -= stone_count);

            if stone == 0 {
                let new_stone = 1;
                self.line
                    .entry(new_stone)
                    .and_modify(|v| *v += stone_count)
                    .or_insert(*stone_count);
            } else {
                let stone_len = stone.to_string().len();
                if stone_len % 2 == 0 {
                    let stone_vec = stone.to_string().chars().collect_vec();

                    let new_stone_1 = String::from_iter(stone_vec[0..(stone_len / 2)].iter())
                        .parse::<usize>()
                        .expect("invalid number");
                    let new_stone_2 =
                        String::from_iter(stone_vec[(stone_len / 2)..stone_len].iter())
                            .parse::<usize>()
                            .expect("Invalid number");

                    self.line
                        .entry(new_stone_1)
                        .and_modify(|v| *v += stone_count)
                        .or_insert(*stone_count);
                    self.line
                        .entry(new_stone_2)
                        .and_modify(|v| *v += stone_count)
                        .or_insert(*stone_count);
                } else {
                    let new_stone = stone * 2024;
                    self.line
                        .entry(new_stone)
                        .and_modify(|v| *v += stone_count)
                        .or_insert(*stone_count);
                }
            }
        }
    }
}
