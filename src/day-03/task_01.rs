use std::fs::read_to_string;

// PROGRAMMERS NOTE: whatever happens with all that "&" & "*", idfk

// declare global constants & variables
const INPUT_FILE : &str = "src/day-03/input";

fn main() {
    let input = read_to_string(INPUT_FILE)
        .expect("cannot read file");

    let banks = input
        .split("\n")
        .collect::<Vec<_>>();

    let mut total_joltage = 0;

    for bank in banks {
        let bank : Vec<i32> = bank
            .chars()
            .map(|x| x.to_string().parse().expect("pls just work idk"))
            .collect();

        let mut joltage_1st : i32 = 0;
        let mut joltage_2nd : i32 = 0;

        for battery in &bank[0..bank.len()-1] {
            if battery > &joltage_1st {
                joltage_1st = *battery;
                joltage_2nd = 0;
            } else if battery > &joltage_2nd {
                joltage_2nd = *battery;
            }
        }
        // now check last element (can only be joltage_2nd)
        if bank.last().unwrap() > &joltage_2nd {
            joltage_2nd = *bank.last().unwrap();
        }

        // get combined joltage
        let joltage_bank : String = joltage_1st.to_string() + &joltage_2nd.to_string();  // note: yeah, it could also be done with 1st * 10, but not today
        let joltage_bank : i32 = joltage_bank
            .parse()
            .expect("cannot parse joltage to i32");

        println!("{joltage_bank}");
        total_joltage += joltage_bank;
    }
    println!("total_joltage: {}", total_joltage);
}