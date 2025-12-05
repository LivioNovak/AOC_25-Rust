// use dependencies
use std::fs::read_to_string;

// declare global constants & variables
const INPUT_FILE: &str = "src/day-01/input";

fn main() {
    let input = read_to_string(INPUT_FILE).expect("cannot read file");

    let mut safe_pos: i32 = 50;
    let mut landed_on_0: i32 = 0;

    for line in input.split("\n") {
        // first, rotate the dial; Note: dial is a circle with 100 numbers
        let steps: i32 = line[1..].parse().expect("Not a number!");
        if line.starts_with('R') {
            // rotate towards right = increasing numbers
            safe_pos = (safe_pos + steps) % 100;
        } else if line.starts_with('L') {
            // rotate towards left = decreasing numbers
            safe_pos = (safe_pos - steps) % 100;
        }
        // increase counter when dial lands on 0
        if safe_pos == 0 {
            landed_on_0 += 1;
        }
    }

    println!("landed on 0: {landed_on_0}-times")
}