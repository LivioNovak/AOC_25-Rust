use std::fs::read_to_string;

// declare global constants and variables
const INPUT_FILE : &str = "src/day-05/input";

fn main() {
    let input = read_to_string(INPUT_FILE)
        .expect("cannot read from input_file");

}

/* TEST

use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello World");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
*/
