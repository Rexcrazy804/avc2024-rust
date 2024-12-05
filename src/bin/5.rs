use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "5";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    //println!("\n=== Part 2 ===");
    //assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part2(input_file)?);
    //println!("Result = {}", result);
    //endregion

    Ok(())
}

struct BeforeAndAfter {
    befores: Option<Vec<usize>>,
    afters: Option<Vec<usize>>,
}

impl BeforeAndAfter {
    fn new_afters(num: usize) -> Self {
        Self {
            befores: None,
            afters: Some(vec![num]),
        }
    }

    fn new_befores(num: usize) -> Self {
        Self {
            befores: Some(vec![num]),
            afters: None,
        }
    }

    fn new(befores: Option<Vec<usize>>, afters: Option<Vec<usize>>) -> Self {
        Self { befores, afters }
    }
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut rules: HashMap<usize, BeforeAndAfter> = HashMap::new();
    let mut answer = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some((data1, data2)) = line.split_once('|') {
            let (data1, data2): (usize, usize) = (data1.parse()?, data2.parse()?);
            process_rules(&mut rules, data1, data2);
            continue;
        }

        if line.is_empty() {
            continue;
        }

        // here the processing of the comma seperated values start
        let vector: Vec<usize> = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        if let Some(middle) = process_vector(&rules, &vector) {
            answer += middle;
        }
    }

    //display_rules(rules);
    Ok(answer)
}

#[allow(dead_code)]
fn display_rules(rules: &HashMap<usize, BeforeAndAfter>) {
    rules.iter().for_each(|(entry, before_after)| {
        let BeforeAndAfter { befores, afters } = before_after;

        println!("value: {entry}");
        println!("Before: {befores:?}");
        println!("Afters: {afters:?}");
    });
}

fn process_vector(rules: &HashMap<usize, BeforeAndAfter>, vector: &Vec<usize>) -> Option<usize> {
    for (index, element) in vector.iter().enumerate() {
        let (before, after) = vector.split_at(index);
        // split at will include the middle element :/
        let after = &after[1..after.len()];

        let Some(BeforeAndAfter { befores, afters }) = rules.get(element) else {
            continue;
        };

        // to put this simply we are checking if an element that exists after our current
        // element exists in the before rule of oru current element
        if let Some(befores) = befores {
            for after_element in after.iter() {
                if befores.contains(after_element) {
                    return None;
                }
            }
        }

        if let Some(afters) = afters {
            for before_element in before.iter() {
                if afters.contains(before_element) {
                    return None;
                }
            }
        }
    }

    let middle = *vector.get(vector.len() / 2)?;

    //println!("vector: {vector:?}\nMID: {middle}");
    Some(middle)
}

fn process_rules(rules: &mut HashMap<usize, BeforeAndAfter>, data1: usize, data2: usize) {
    // data 2 will be part of the afters for data 1 here:
    if let Some(BeforeAndAfter { befores, afters }) =
        rules.insert(data1, BeforeAndAfter::new_afters(data2))
    {
        let new_afters = match afters {
            None => Some(vec![data2]),
            Some(mut afters) => {
                if !afters.contains(&data2) {
                    afters.push(data2);
                    Some(afters)
                } else {
                    Some(afters)
                }
            }
        };

        rules.insert(data1, BeforeAndAfter::new(befores, new_afters));
    }

    // data 1 will be part of the befores for data 1 here:
    if let Some(BeforeAndAfter { befores, afters }) =
        rules.insert(data2, BeforeAndAfter::new_befores(data1))
    {
        let new_befores = match befores {
            None => Some(vec![data1]),
            Some(mut befores) => {
                if !befores.contains(&data1) {
                    befores.push(data1);
                    Some(befores)
                } else {
                    Some(befores)
                }
            }
        };

        rules.insert(data2, BeforeAndAfter::new(new_befores, afters));
    }
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    todo!()
}
