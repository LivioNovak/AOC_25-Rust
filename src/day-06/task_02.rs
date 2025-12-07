use std::fs::read_to_string;

// declare global constants and variables
const INPUT_FILE : &str = "src/day-06/input";

fn main() {
    let input = read_to_string(INPUT_FILE)
        .expect("cannot read from input_file");
    let input: Vec<_> = input
        .split("\n")
        .collect();

    // get operations
    let operations : Vec<&str> = get_without_blanks(input
        .last()
        .unwrap());

    let input: Vec<Vec<char>> = input
        .iter()
        .map(|x| x.chars().collect())
        .collect();

    let mut cephalopod_nums : Vec<Vec<char>> = Vec::new();

    let mut max_line_length = 0;
    for line in &input {
        if max_line_length < line.len() {
            max_line_length = line.len();
        }
    }

    // read numbers according to cephalopod math (columnwise -> switch rows and columns)
    for j in 0..max_line_length {
        cephalopod_nums.push(Vec::new());
        // exclude last line containing the operations
        for i in 0..input.len() -1 {
            if j < input[i].len() {
                cephalopod_nums[j].push(input[i][j])
            }
        }
    }

    let mut total_sum_problems = 0u64;
    let mut current_op_pos = 0;
    let mut current_answer = 0;
    if operations[0] == "*" {
        current_answer = 1;
    }
    for num in cephalopod_nums {
        let num : String = num.iter().collect();
        let num = num.trim();

        // when encountering an empty "number", the problem is finished
        if num == "" {
            total_sum_problems += &current_answer;   // add answer to total

            current_op_pos += 1;                     // next problem, next operator
            if operations[current_op_pos] == "*" {
                current_answer = 1;                   // multiplication -> reset answer to 1
            } else if operations[current_op_pos] == "+" {
                current_answer = 0;                  // addition -> reset answer to 0
            }
        } else {
            // current problem ongoing
            let value : u64 = num
                .parse()
                .expect("cannot parse to u64");

            if operations[current_op_pos] == "*" {
                current_answer *= value;
            } else if operations[current_op_pos] == "+" {
                current_answer += value;
            }
        }
    }
    // add last answer also to total
    total_sum_problems += current_answer;

    println!("total: {}", total_sum_problems);
}

fn get_without_blanks(input : &str) -> Vec<&str> {
    let mut output : Vec<&str> = Vec::new();
    for element in input.split(" ") {
        if element != "" {
            output.push(element);
        }
    }

    output
}