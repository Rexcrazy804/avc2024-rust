use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

const DAY: &str = "7";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);
    println!("TEST PASSED");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    //println!("\n=== Part 2 ===");
    //assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    //println!("TEST PASSED");
    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part2(input_file)?);
    //println!("Result = {}", result);
    //endregion

    Ok(())
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut equations: Vec<Equation> = Vec::new();

    for line in reader.lines() {
        let equation = Equation::parse(&line?)?;
        equations.push(equation);
    }

    let answer = equations
        .iter()
        .filter_map(|x| x.calibrate())
        .fold(0, |acc, x| acc + x);

    Ok(answer)
}

struct Equation {
    target: usize,
    operands: Vec<usize>,
}

#[derive(Clone, Copy, Debug)]
enum Operators {
    Add,
    Multiply,
}

impl Equation {
    fn parse(line: &str) -> Result<Self> {
        use std::result::Result;
        let (target, operands) = line.split_once(':').unwrap();

        let target = target.parse::<usize>()?;

        let operands = operands
            .trim()
            .split(' ')
            .filter_map(|x| {
                let Result::Ok(x) = x.parse::<usize>() else {
                    eprintln!("failed to parse {x}");
                    return None;
                };
                Some(x)
            })
            .collect::<Vec<usize>>();

        Ok(Self { target, operands })
    }

    fn calibrate(&self) -> Option<usize> {
        let operator_count = self.operands.len() - 1;

        let combinations = Self::get_operator_combinations(operator_count);

        for combination in &combinations {
            let mut result = self.operands[0];
            for (index, operator) in combination.iter().enumerate() {
                match operator {
                    Operators::Add => result = result + self.operands[index + 1],
                    Operators::Multiply => result = result * self.operands[index + 1],
                }
            }

            if result == self.target {
                println!("Valid: {self}");
                return Some(self.target);
            }
        }

        println!("Invalid: {self}");
        None
    }

    fn get_operator_combinations(operator_count: usize) -> Vec<Vec<Operators>> {
        use itertools::Itertools;
        let operators = [Operators::Add, Operators::Multiply];
        if operator_count == 1 {
            return vec![vec![Operators::Add], vec![Operators::Multiply]];
        }

        let combinations: Vec<Vec<Operators>> = (2..operator_count).fold(
            operators
                .iter()
                .cartesian_product(operators.iter())
                .map(|(&a, &b)| Vec::from([a, b]))
                .collect(),
            |acc, _| {
                acc.into_iter()
                    .cartesian_product(operators.iter())
                    .map(|(mut a, b)| {
                        a.push(*b);
                        a
                    })
                    .collect()
            },
        );

        combinations
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Target: {} ", self.target)?;
        write!(f, "Operands: ")?;
        for operand in &self.operands {
            write!(f, "{operand} ")?;
        }
        std::result::Result::Ok(())
    }
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    todo!()
}
