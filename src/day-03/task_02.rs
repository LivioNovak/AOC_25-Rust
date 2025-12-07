use std::cmp::{min};
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

    let mut total_joltage : i128 = 0;

    for bank in banks {
        let bank : Vec<i32> = bank
            .chars()
            .map(|x| x.to_string().parse().expect("pls just work idk"))
            .collect();

        let mut current_pos = 0;
        let mut activated_batteries = [0i32; 12];
        for battery in &bank[0..bank.len()] {
            // e.g. when there's only 8 batteries left, 4 batteries can't be changed anymore
            let max_array_pos = min(12, bank.len() - current_pos);
            println!("{battery}\t{max_array_pos}");
            for i in 0 + (12-max_array_pos)..12 {
                if activated_batteries[i] < *battery {
                    // found a new higher number
                    activated_batteries[i] = *battery;
                    // set every joltage behind to 0
                    for j in i+1..12 {
                        activated_batteries[j] = 0;
                    }
                    break;
                }
                //println!("{}\t{battery}", activated_batteries[i]);
            }
            current_pos += 1;
        }
        for i in activated_batteries {
            println!("{i}");
        }
        // now check last element (can only be joltage_2nd)
        /*if bank.last().unwrap() > &joltage_2nd {
            joltage_2nd = *bank.last().unwrap();
        }*/

        // get combined joltage

        let base :i64 = 10;
        let mut joltage_bank : i128 = 0;
        for i in 0..12 {
            joltage_bank += activated_batteries[11-i] as i128 * (base.pow(i as u32) as i128);
        }
        println!("bank: {}",joltage_bank);
        total_joltage += joltage_bank;
    }
    println!("total_joltage: {}", total_joltage);
}