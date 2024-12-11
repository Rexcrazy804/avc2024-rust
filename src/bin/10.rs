use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools; //use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, isize, usize};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);
    println!("TEST PASSED");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);
    println!("TEST PASSED");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut trail_heads: Vec<TrailHead> = Vec::new();
    for (row_index, line) in reader.lines().enumerate() {
        let line = line?;
        let row = line
            .chars()
            .enumerate()
            .map(|(col_index, x)| {
                let x = x.to_digit(10).expect("invalid digit") as usize;
                if x == 0 {
                    trail_heads.push(TrailHead::new((row_index, col_index)));
                }
                x
            })
            .collect_vec();
        map.push(row);
    }

    let answer = trail_heads.iter_mut().fold(0, |answer, head| {
        head.calculate(&map);
        answer + head.get_score()
    });

    Ok(answer)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut trail_heads: Vec<TrailHead> = Vec::new();
    for (row_index, line) in reader.lines().enumerate() {
        let line = line?;
        let row = line
            .chars()
            .enumerate()
            .map(|(col_index, x)| {
                let x = x.to_digit(10).expect("invalid digit") as usize;
                if x == 0 {
                    trail_heads.push(TrailHead::new((row_index, col_index)));
                }
                x
            })
            .collect_vec();
        map.push(row);
    }

    let answer = trail_heads.iter_mut().fold(0, |answer, head| {
        head.calculate(&map);
        answer + head.get_rating()
    });

    Ok(answer)
}

struct TrailHead {
    head: (usize, usize),
    score: HashSet<(usize, usize)>,
    rating: HashSet<Vec<(usize, usize)>>,
}

impl TrailHead {
    fn new(head: (usize, usize)) -> Self {
        TrailHead {
            head,
            score: HashSet::new(),
            rating: HashSet::new(),
        }
    }

    fn get_score(&self) -> usize {
        self.score.len()
    }

    fn get_rating(&self) -> usize {
        //dbg!(&self.rating);
        self.rating.len()
    }

    fn calculate(&mut self, map: &Vec<Vec<usize>>) {
        self.hike(map, self.head, vec![]);
    }

    fn hike(
        &mut self,
        map: &Vec<Vec<usize>>,
        position: (usize, usize),
        mut path: Vec<(usize, usize)>,
    ) {
        let current_value = map[position.0][position.1];
        path.push(position);
        //dbg!(current_value);
        //dbg!(position);
        if current_value == 9 {
            self.score.insert(position);
            self.rating.insert(path);
            return;
        }

        for direction in DIRECTIONS {
            match direction {
                Direction::UP => {
                    if position.0 == 0 {
                        continue;
                    }
                    let next_position = (position.0 - 1, position.1);
                    if map[next_position.0][next_position.1] != current_value + 1 {
                        continue;
                    }
                    //dbg!(next_position);
                    self.hike(map, next_position, path.clone());
                }
                Direction::Down => {
                    if position.0 == map.len() - 1 {
                        continue;
                    }
                    let next_position = (position.0 + 1, position.1);
                    if map[next_position.0][next_position.1] != current_value + 1 {
                        continue;
                    }
                    self.hike(map, next_position, path.clone());
                }
                Direction::Left => {
                    if position.1 == 0 {
                        continue;
                    }
                    let next_position = (position.0, position.1 - 1);
                    if map[next_position.0][next_position.1] != current_value + 1 {
                        continue;
                    }
                    self.hike(map, next_position, path.clone());
                }
                Direction::Right => {
                    if position.1 == map[0].len() - 1 {
                        continue;
                    }
                    let next_position = (position.0, position.1 + 1);
                    if map[next_position.0][next_position.1] != current_value + 1 {
                        continue;
                    }
                    self.hike(map, next_position, path.clone());
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    UP,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
