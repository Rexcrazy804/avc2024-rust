use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{isize, usize};

const DAY: &str = "8";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    //println!("=== Part 1 ===");
    //assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);
    //println!("TEST PASSED");
    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part1(input_file)?);
    //println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);
    println!("TEST PASSED");
    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part2(input_file)?);
    //println!("Result = {}", result);
    //endregion

    Ok(())
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut antena = Antena::default();

    let mut map_clone: Vec<String> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        antena.insert_antenas(&line, index);
        map_clone.push(line);
    }

    //println!("{:?}", antena.antenas);
    antena.collect_antinodes();

    let answer = antena.antinodes.len();

    for (index_row, line) in map_clone.iter().enumerate() {
        for (index_col, char) in line.chars().enumerate() {
            if antena.antinodes.contains(&(index_row, index_col)) {
                print!("#");
                continue;
            }
            print!("{}", char);
        }
        print!("\n");
    }

    //println!("{}", antena.index_limit);

    Ok(answer)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    todo!()
}

#[derive(Default)]
struct Antena {
    antenas: HashMap<char, HashSet<(usize, usize)>>,
    antinodes: HashSet<(usize, usize)>,
    index_limit: usize,
}

impl Antena {
    fn insert_antenas(&mut self, line: &str, row_index: usize) {
        // this could have been a constant 50 but the value in the sample test is 12:12 so yeah
        if row_index > self.index_limit {
            self.index_limit = row_index
        }

        for (col_index, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            self.antenas
                .entry(char)
                .and_modify(|x| {
                    x.insert((row_index, col_index));
                })
                .or_insert({
                    let mut set: HashSet<(usize, usize)> = HashSet::new();
                    set.insert((row_index, col_index));
                    set
                });
        }
    }

    fn collect_antinodes(&mut self) {
        for positions in self.antenas.values() {
            let unique_pair_of_positions = iproduct!(positions.iter(), positions.iter()).unique();

            for (position_1, position_2) in unique_pair_of_positions {
                let antinode_positions = self.get_valid_antinodes(position_1, position_2);

                antinode_positions.iter().for_each(|position| {
                    if let Some(position) = position {
                        let _ = &self.antinodes.insert(*position);
                    }
                });
            }
        }
    }

    fn get_valid_antinodes(
        &self,
        position_1: &(usize, usize),
        position_2: &(usize, usize),
    ) -> [Option<(usize, usize)>; 2] {
        let position_1 = (position_1.0 as isize, position_1.1 as isize);

        let position_2 = (position_2.0 as isize, position_2.1 as isize);

        let difference = (position_2.0 - position_1.0, position_2.1 - position_1.1);

        let mirror_1 = (position_2.0 + difference.0, position_2.1 + difference.1);

        let mirror_2 = (position_1.0 - difference.0, position_1.1 - difference.1);

        [
            self.validate(mirror_1, difference),
            self.validate(mirror_2, difference),
        ]
    }

    fn validate(&self, mirror: (isize, isize), diff: (isize, isize)) -> Option<(usize, usize)> {
        if diff.0 < 1 && diff.1 < 1 {
            return None;
        } else if mirror.0 < 0 || mirror.1 < 0 {
            None
        } else if mirror.0 > self.index_limit as isize || mirror.1 > self.index_limit as isize {
            None
        } else {
            Some((mirror.0 as usize, mirror.1 as usize))
        }
    }
}
