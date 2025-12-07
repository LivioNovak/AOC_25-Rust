use std::fs::read_to_string;

// declare global constants and variables
const INPUT_FILE : &str = "src/day-05/input";

fn main() {
    let input = read_to_string(INPUT_FILE)
        .expect("cannot read from input_file");
    let input : Vec<&str> = input
        .split("\n\n")
        .collect();

    let id_ranges = input[0];
    let ids = input[1];

    // map the string to u64 values
    let ids : Vec<u64> = ids
        .split("\n")
        .map(|x| x.parse().expect("cannot parse ids to u64"))
        .collect();

    let id_ranges : Vec<Vec<u64>> = id_ranges
        .split("\n")
        .map(|x| x
            .split("-")
            .map(|y| y.parse().expect("cannot parse ranges to u64"))
            .collect())
        .collect();

    let mut fresh_id_counter = 0;

    for id in ids {
        for range in &id_ranges {
            if id >= range[0] && id <= range[1] {
                // id is inside an id-range => fresh
                fresh_id_counter += 1;
                break;
            }
        }
    }

    println!("# fresh ids: {}", fresh_id_counter);
}
