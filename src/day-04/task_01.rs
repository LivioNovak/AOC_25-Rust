use std::cmp::{max, min};
use std::fs::read_to_string;

// declare global constants & variables
const INPUT_FILE : &str = "src/day-04/input";

fn main() {
    let input = read_to_string(INPUT_FILE)
        .expect("cannot read from input_file");
    let input : Vec<&str> = input
        .split("\n")
        .collect();

    // map input to integer-grid
    let grid = input_map_to_vec(input);
    let mut accessible_papers : i32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // if selected element is a paper, check surrounding elements
            if grid[i][j] == 1 {
                let mut papers_nearby : i8 = 0;     // increasing counter for nearby papers
                // Set the borders for nearby elements (minimal_x, maximum_x, minimal_y, maximum_y)
                let borders : (usize, usize, usize, usize) =
                    (max(i as isize -1, 0) as usize,
                     min(i+1, grid.len() -1),
                     max(j as isize -1, 0) as usize,
                     min(j+1, grid[i].len() -1));

                for x in borders.0..=borders.1 {
                    for y in borders.2..=borders.3 {
                        // don't count the selected paper itself (x=0,y=0)
                        if !(x == i && y == j) {
                            papers_nearby += grid[x][y];

                        }
                    }
                }
                // when less than 4 papers are nearby, increase accessible_papers_counter by one
                if papers_nearby < 4 {
                    accessible_papers += 1;
                }
            }
        }
    }

    println!("papers accessible: {}", accessible_papers);
}

fn input_map_to_vec(input: Vec<&str>) -> Vec<Vec<i8>> {
    // len = (#rows, #cols)
    let len: (usize, usize) = (input.len(), input.first().unwrap().len());
    let mut grid : Vec<Vec<i8>> = vec![vec![0i8; len.0]; len.1];
    /* map strings to i8
        @ ... 1
        . ... 0
     */
    for i in 0..input.len() {
        // convert string to collection of chars
        let row : Vec<char> = input[i].chars().collect();
        for j in 0..row.len() {
            if row[j] == '@' {
                grid[i][j] = 1;
            } else {
                grid[i][j] = 0;
            }
        }
    }
    // return the grid
    grid
}