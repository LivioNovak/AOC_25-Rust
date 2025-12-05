// use dependencies
use std::fs::read_to_string;

// declare global constants & variables
const INPUT_FILE: &str = "src/day-01/input";

fn main() {
    let input = read_to_string(INPUT_FILE).expect("cannot read file");

    let mut safe_pos: i32 = 50;
    let mut passed_0: i32 = 0;

    for line in input.split("\n") {
        let prev_pos = safe_pos;
        let steps: i32 = line[1..].parse().expect("Not a number!");

        // first, rotate the dial; Note: dial is a circle with 100 numbers
        if line.starts_with('R') {
            // rotate towards right = increasing numbers
            let calculated_pos = safe_pos + steps;
            // calculate times 0 got passed
            passed_0 += calculated_pos / 100;
            safe_pos = (safe_pos + steps) % 100;     // actual position
        } else if line.starts_with('L') {
            // rotate towards left = decreasing numbers
            let calculated_pos = safe_pos - steps;
            // calculate times 0 got passed
            if prev_pos > 0 && calculated_pos <= 0 {
                passed_0 += 1;
            }
            // also count full rotations
            passed_0 += (calculated_pos / 100).abs();
            safe_pos = (safe_pos - steps) % 100;     // actual position
            if safe_pos < 0 {
                safe_pos += 100;
            }
        }
    }
    println!("passed by 0: {passed_0}-times")
}