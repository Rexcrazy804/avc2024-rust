use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "6";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);
    println!("TEST PASSED");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    //println!("\n=== Part 2 ===");
    //assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);
    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part2(input_file)?);
    //println!("Result = {}", result);
    //endregion

    Ok(())
}

const PLAYER_CHARACTER: char = '^';
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut player_start_pos: Option<(usize, usize)> = None;

    for (row_index, line) in reader.lines().enumerate() {
        let tiles = line?.chars().collect::<Vec<char>>();

        if player_start_pos.is_none() {
            //dbg!(&tiles);
            // could potentially optimize this if I can find an std function
            // that can search and return the index of the PLAYER_CHARACTER
            // I am looking for
            for (col_index, tile) in tiles.iter().enumerate() {
                if *tile == PLAYER_CHARACTER {
                    player_start_pos = Some((row_index, col_index));
                }
            }
        }

        map.push(tiles);
    }

    let initial_direction = 0;
    let Some(player_pos) = player_start_pos else {
        panic!("Player not found")
    };
    let mut tiles_covered: HashSet<(usize, usize)> = HashSet::new();

    move_guard(initial_direction, player_pos, &map, &mut tiles_covered);
    let answer = tiles_covered.len();

    Ok(answer)
}

fn move_guard(
    mut direction: usize,
    player_pos: (usize, usize),
    map: &Vec<Vec<char>>,
    tiles_covered: &mut HashSet<(usize, usize)>,
) {
    tiles_covered.insert(player_pos);

    let max_rows = map.len();
    let max_cols = map[0].len();

    //dbg!(direction);
    //dbg!(player_pos);

    // 1. insert current position
    // 1.5 check if next position is valid, if not EXIT RECURSION
    // 2. check if next position is an obstacles
    // 3. if 2. is yes {
    //  incremenet direction
    //  goto 1.5
    // }
    // 4. RECURSION POINT update player_pos to next_pos and call the function again (1.)

    loop {
        let next_position = match &DIRECTIONS[direction] {
            Direction::Up => {
                if player_pos.0 < 1 {
                    return;
                }
                (player_pos.0 - 1, player_pos.1)
            }
            Direction::Right => {
                if player_pos.1 == max_cols - 1 {
                    return;
                }
                (player_pos.0, player_pos.1 + 1)
            }
            Direction::Down => {
                if player_pos.0 == max_rows - 1 {
                    return;
                }
                (player_pos.0 + 1, player_pos.1)
            }
            Direction::Left => {
                if player_pos.1 < 1 {
                    return;
                }
                (player_pos.0, player_pos.1 - 1)
            }
        };

        if is_barricade(next_position, map).is_none() {
            direction = if direction == 3 { 0 } else { direction + 1 };
        } else {
            move_guard(direction, next_position, map, tiles_covered);
            break;
        }
    }
}

fn is_barricade(next_position: (usize, usize), map: &Vec<Vec<char>>) -> Option<()> {
    if *map.get(next_position.0)?.get(next_position.1)? != '#' {
        Some(())
    } else {
        None
    }
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    todo!()
}
