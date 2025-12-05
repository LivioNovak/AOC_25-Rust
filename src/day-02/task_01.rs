use std::fs::read_to_string;

// declare global constants & variables
const INPUT_FILE : &str = "src/day-02/input";

fn main() {
    let input = read_to_string(INPUT_FILE)
        .expect("cannot read file");

    let input = input.trim().split(",").collect::<Vec<_>>();

    let mut sum_invalid = 0;


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
            if id.ilog10() % 2 == 1 {
                // get "1st half of the digit-sequence" (e.g. 123456 -> 123)
                let base : f64 = 10.0;
                let half_power = (id.ilog10() / 2) as i32 + 1;
                let half = id / (base.powi(half_power) as i64);
                // println!("{}\t{}\t{}\t{}\t{}\t{}", id, half, half_power, sum_invalid, lowest, highest);
                if half + (half * (base.powi(half_power) as i64)) == id {
                    // i is made of digit-sequence repeated twice => invalid
                    sum_invalid += id;
                    println!("invalid");
                }
            }
        }
    }

    println!("{}", sum_invalid);
}