use std::fs::read_to_string;

// declare global constants and variables
const INPUT_FILE : &str = "src/day-05/input";

fn main() {
    let input = read_to_string(INPUT_FILE)
        .expect("cannot read from input_file");
    let input : Vec<&str> = input
        .split("\n\n")
        .collect();

    let id_ranges : Vec<Vec<u64>> = input[0]
        .split("\n")
        .map(|x| x
            .split("-")
            .map(|y| y.parse().expect("cannot parse ranges to u64"))
            .collect())
        .collect();

    let mut fresh_id_counter: u128 = 0;

    let mut checked_ranges : Vec<(u64, u64)> = Vec::new();
    // format: (lowest_border, highest_border)

    for range_sel in &id_ranges {
        // check if selected range is part of an already checked range
        let mut is_checked = false;
        for checked in &checked_ranges {
            if range_sel[0] >= checked.0 && range_sel[1] <= checked.1 {
                is_checked = true;
            }
        }
        // skip this range if already checked
        if is_checked {
            continue;
        }

        /*
        otherwise, combine all ranges that are overlapping with currently selected
        (directly and indirectly) into one big range
        */
        let mut big_range : (u64, u64) = (range_sel[0], range_sel[1]);
        let mut something_added = true;
        while something_added {
            /*
            when an other range gets added, it's possible that now other ranges previously
            not overlapping now do overlap -> check everything again
            */
            something_added = false;
            for other_range in &id_ranges {
                // overlapping with big_ranges lower border:
                if big_range.0 <= other_range[1] && big_range.0 > other_range[0] {
                    big_range.0 = other_range[0];
                    something_added = true;
                }
                // overlapping with big_ranges higher border:
                if big_range.1 >= other_range[0] && big_range.1 < other_range[1] {
                    big_range.1 = other_range[1];
                    something_added = true;
                }
            }
        }


        // count elements inside big_range and add it to fresh_ids
        fresh_id_counter += (big_range.1 - big_range.0 + 1) as u128;  // +1: count border-el as well
        // add big_range to already checked ranges
        checked_ranges.push(big_range);
    }

    //debug: sort checked ranges
    checked_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    for range in checked_ranges  {
        println!("range: {}\t{}", range.0, range.1);
    }


    println!("# fresh ids: {}", fresh_id_counter);
}