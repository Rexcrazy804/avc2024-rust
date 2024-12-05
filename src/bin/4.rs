use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "4";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

const SEARCH_WORD: &'static str = "XMAS";

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut answer = 0;
    let the_matrix: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|x| {
            let std::result::Result::Ok(x) = x else {
                return None;
            };
            Some(x.chars().collect::<Vec<char>>())
        })
        .collect();

    for (row_num, rows) in the_matrix.iter().enumerate() {
        for (col_num, &col_char) in rows.iter().enumerate() {
            if col_char == 'X' {
                let no_subtract_row = row_num < 1;
                let no_subtract_col = col_num < 1;

                if !no_subtract_row
                    && find_xmas(
                        &the_matrix,
                        row_num - 1,
                        col_num,
                        String::from('X'),
                        Direction::Up,
                    )
                {
                    answer += 1;
                }
                if find_xmas(
                    &the_matrix,
                    row_num + 1,
                    col_num,
                    String::from('X'),
                    Direction::Down,
                ) {
                    answer += 1;
                }
                if !no_subtract_col
                    && find_xmas(
                        &the_matrix,
                        row_num,
                        col_num - 1,
                        String::from('X'),
                        Direction::Left,
                    )
                {
                    answer += 1;
                }
                if find_xmas(
                    &the_matrix,
                    row_num,
                    col_num + 1,
                    String::from('X'),
                    Direction::Right,
                ) {
                    answer += 1;
                }
                if !no_subtract_row
                    && find_xmas(
                        &the_matrix,
                        row_num - 1,
                        col_num + 1,
                        String::from('X'),
                        Direction::TopRight,
                    )
                {
                    answer += 1;
                }
                if !no_subtract_row
                    && !no_subtract_col
                    && find_xmas(
                        &the_matrix,
                        row_num - 1,
                        col_num - 1,
                        String::from('X'),
                        Direction::TopLeft,
                    )
                {
                    answer += 1;
                }
                if find_xmas(
                    &the_matrix,
                    row_num + 1,
                    col_num + 1,
                    String::from('X'),
                    Direction::BottomRight,
                ) {
                    answer += 1;
                }
                if !no_subtract_col
                    && find_xmas(
                        &the_matrix,
                        row_num + 1,
                        col_num - 1,
                        String::from('X'),
                        Direction::BottomLeft,
                    )
                {
                    answer += 1;
                }
            }
        }
    }

    Ok(answer)
}

fn find_xmas(
    matrix: &Vec<Vec<char>>,
    mut row_num: usize,
    mut col_num: usize,
    mut buffer: String,
    direction: Direction,
) -> bool {
    let Some(row) = matrix.get(row_num) else {
        return false;
    };
    let Some(char) = row.get(col_num) else {
        return false;
    };
    buffer.push(*char);

    if !SEARCH_WORD.contains(&buffer) {
        return false;
    }

    if buffer.len() >= SEARCH_WORD.len() {
        if buffer == SEARCH_WORD {
            return true;
        } else {
            return false;
        }
    }

    if let Direction::Left | Direction::BottomLeft | Direction::TopLeft = direction {
        if col_num < 1 {
            return false;
        }
    }
    if let Direction::TopLeft | Direction::Up | Direction::TopRight = direction {
        if row_num < 1 {
            return false;
        }
    }

    match direction {
        Direction::Up => {
            row_num = row_num - 1;
        }
        Direction::Down => {
            row_num = row_num + 1;
        }
        Direction::Left => {
            col_num = col_num - 1;
        }
        Direction::Right => {
            col_num = col_num + 1;
        }
        Direction::TopRight => {
            row_num = row_num - 1;
            col_num = col_num + 1;
        }
        Direction::TopLeft => {
            row_num = row_num - 1;
            col_num = col_num - 1;
        }
        Direction::BottomRight => {
            row_num = row_num + 1;
            col_num = col_num + 1;
        }
        Direction::BottomLeft => {
            row_num = row_num + 1;
            col_num = col_num - 1;
        }
    };

    find_xmas(matrix, row_num, col_num, buffer.clone(), direction)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut answer = 0;
    let the_matrix: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|x| {
            let std::result::Result::Ok(x) = x else {
                return None;
            };
            Some(x.chars().collect::<Vec<char>>())
        })
        .collect();

    for (row_num, rows) in the_matrix.iter().enumerate() {
        for (col_num, &col_char) in rows.iter().enumerate() {
            if col_char == 'A' {
                if let Some(()) = find_cross_mas(&the_matrix, row_num, col_num) {
                    answer += 1;
                }
            }
        }
    }

    Ok(answer)
}

fn find_cross_mas(matrix: &Vec<Vec<char>>, row_num: usize, col_num: usize) -> Option<()> {
    if row_num < 1 {
        return None;
    }
    if col_num < 1 {
        return None;
    }

    let bottom_row = matrix.get(row_num + 1)?;
    let bottom_right_char = bottom_row.get(col_num + 1)?;
    let bottom_left_char = bottom_row.get(col_num - 1)?;

    let top_row = matrix.get(row_num - 1)?;
    let top_right_char = top_row.get(col_num + 1)?;
    let top_left_char = top_row.get(col_num - 1)?;

    let diagonal_one = String::from_iter([bottom_left_char, &'A', top_right_char]);
    let diagonal_two = String::from_iter([top_left_char, &'A', bottom_right_char]);

    if ("MAS" == diagonal_one || "SAM" == diagonal_one)
        && ("MAS" == diagonal_two || "SAM" == diagonal_two)
    {
        Some(())
    } else {
        None
    }
}
