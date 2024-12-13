use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);
    println!("Test Passed");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    // todo!("Set the expected answer for the test input");
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut garden_map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        garden_map.push(line.chars().collect_vec());
    }

    let mut garden_regions: HashMap<char, Vec<Regions>> = HashMap::new();

    for (row_index, row) in garden_map.iter().enumerate() {
        for (col_index, plant_type) in row.iter().enumerate() {
            garden_regions
                .entry(*plant_type)
                .and_modify(|regions| {
                    for region in regions.iter() {
                        if region.plots.contains(&(row_index, col_index)) {
                            return;
                        }
                    }

                    let mut new_region = Regions::new(*plant_type);
                    new_region.get_region_plots((row_index, col_index), &garden_map);
                    regions.push(new_region);
                })
                .or_insert({
                    let mut new_region = Regions::new(*plant_type);
                    new_region.get_region_plots((row_index, col_index), &garden_map);
                    vec![new_region]
                });
        }
    }

    let answer = garden_regions.values().fold(0, |answer, regions| {
        let mut answer = answer;
        for region in regions {
            //println!("REGION: {}, AREA: {}, PER: {}", region.plant_type, region.get_area(), region.perimetre);
            answer += region.perimetre * region.get_area()
        }
        answer
    });

    Ok(answer)
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn part2<R: BufRead>(reader: R) -> Result<usize> {
    todo!()
}

struct Regions {
    plant_type: char,
    perimetre: usize,
    plots: HashSet<(usize, usize)>,
}

impl Regions {
    fn new(plant_type: char) -> Self {
        Self {
            plant_type,
            perimetre: 0,
            plots: HashSet::new(),
        }
    }

    fn get_region_plots(&mut self, plant_position: (usize, usize), garden_map: &Vec<Vec<char>>) {
        if garden_map[plant_position.0][plant_position.1] != self.plant_type {
            self.perimetre += 1;
            return;
        }

        if !self.plots.insert(plant_position) {
            return;
        }

        if plant_position.0 != 0 {
            let next_position = (plant_position.0 - 1, plant_position.1);
            self.get_region_plots(next_position, garden_map);
        } else {
            self.perimetre += 1
        }

        if plant_position.0 != garden_map.len() - 1 {
            let next_position = (plant_position.0 + 1, plant_position.1);
            self.get_region_plots(next_position, garden_map);
        } else {
            self.perimetre += 1
        }

        if plant_position.1 != 0 {
            let next_position = (plant_position.0, plant_position.1 - 1);
            self.get_region_plots(next_position, garden_map);
        } else {
            self.perimetre += 1
        }

        if plant_position.1 != garden_map[0].len() - 1 {
            let next_position = (plant_position.0, plant_position.1 + 1);
            self.get_region_plots(next_position, garden_map);
        } else {
            self.perimetre += 1
        }
    }

    fn get_area(&self) -> usize {
        self.plots.len()
    }
}
