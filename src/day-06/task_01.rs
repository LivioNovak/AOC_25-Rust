use std::fs::read_to_string;

// declare global constants and variables
const INPUT_FILE : &str = "src/day-06/input";

fn main() {
    let input = read_to_string(INPUT_FILE)
        .expect("cannot read from input_file");
    let input: Vec<_> = input
        .split("\n")
        .collect();

    let operations : Vec<&str> = get_without_blanks(input
        .last()
        .unwrap());

    let mut numbers : Vec<Vec<&str>> = Vec::new();

    // get numbers of each line (without the blanks in lines) & save them in numbers
    for i in 0..input.len() -1 {
        let line = input[i];
        numbers.push(get_without_blanks(line));
    }

    // map each elem from strings to u64
    let numbers : Vec<Vec<u64>> = numbers
        .iter()
        .map(|line| line
            .iter()
            .map(|elem| elem.parse().expect("cannot parse to u64"))
            .collect())
        .collect();

    //format: (operator, Vector of every number)
    let mut problems : Vec<(&str, Vec<u64>)> = Vec::new();

    // write operations to problems & initialize each problem
    for i in 0..operations.len() {
        problems.push((operations[i], Vec::new()));
    }

    // separate problems by writing every column into problems
    for i in 0..numbers.len() {
        for j in 0..numbers[0].len() {
            problems[j].1.push(numbers[i][j]);
        }
    }


    let mut total_sum_problems :u128 = 0;

    for problem in problems {
        let mut answer :u64 = 0;
        if problem.0 == "+" {
            for num in problem.1 {
                answer += num;
            }
        } else if problem.0 == "*" {
            answer = 1; // must start with 1
            for num in problem.1 {
                answer *= num;
            }
        }
        println!("answer: {}", answer);
        // add this problems answer to total
        total_sum_problems += answer as u128;
    }

    // print total
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
