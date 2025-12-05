use std::fs::read_to_string;

// declare global constants & variables
const INPUT_FILE : &str = "src/day-02/input";

fn main() {
    let input = read_to_string(INPUT_FILE)
        .expect("cannot read file");

    let input = input
        .trim()
        .split(",")
        .collect::<Vec<_>>();

    let mut sum_invalid :i64 = 0;


    for part_range in input {
        let borders = part_range
            .split("-")
            .collect::<Vec<_>>();

        // only used for range of for-loop
        let lowest : i64 = borders[0]
            .parse()
            .expect("cannot parse lowest to i32");
        let highest : i64 = borders[1]
            .parse()
            .expect("cannot parse highest to i32");

        for id in lowest..highest {
            // check if invalid (can only occur by even digit-sequence-lengths)
            let id = id.to_string();
            let length = id.len();

            for pattern_size in 1..length {
                if length % pattern_size == 0 {
                    let pattern = &id[0..pattern_size];
                    let times = length / pattern_size;
                    let mut occurence = 0;
                    for i in 0..times {
                        let lower = pattern_size * i;
                        let higher = pattern_size * i + pattern_size;
                        if *pattern == id[lower..higher] {
                            occurence += 1;
                        }
                    }
                    if occurence * pattern_size == length {
                        let id_value : i64 = id.parse().expect("cannot parse back to number");
                        sum_invalid += id_value;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", sum_invalid);
}