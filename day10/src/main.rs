use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading this file");

    println!("Finished with an error score of {}", task_1(contents));
}

fn task_1(input: String) -> i32 {
    let mut stack: Vec<char> = Vec::new();
    let mut expected_value: Option<char>;
    let mut line_score = 0;

    for line in input.lines() {
        for c in line.chars() {
            if c == '[' || c == '<' || c == '(' || c == '{' {
                stack.push(c);
            }
            else {
                expected_value = stack.pop();
    
                if expected_value != closer_to_opener(c)
                {
                    line_score += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0
                    }
                }
            }
        }
    }

    return line_score;
}

fn closer_to_opener(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '>' => Some('<'),
        '}' => Some('{'),
        _ => None,
    }
}