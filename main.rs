use regex::Regex;

use std::io;
use std::io::Write;


fn main() {

    // Selects all first actions from an expression
    let re_first_actions = r"([\-\+]\d[\*/]\d)([\*/]\d)*";
    let re_numbers = r"\d+";
    let re_operators = r"[\*/\-\+]";

    let mut expression = String::new();

    print!("Enter expression: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut expression).unwrap();

    // Adds a "+"" to the beginning of an expression
    if expression.chars().next().unwrap() != '-' {
        expression = "+".to_owned() + expression.as_str();
    }

    let mut result: i32 = 0;

    let first_actions = re_split(expression.as_str(), re_first_actions);

    for action in first_actions {
        let numbers = re_split(action.as_str(), re_numbers);
        let operators = re_split(action.as_str(), re_operators);
        result += calculate_subexpression(numbers, operators);
    }

    let new_expression = re_remove(expression.as_str(), re_first_actions);

    if new_expression != "\n" {
        let numbers = re_split(new_expression.as_str(), re_numbers);
        let operators = re_split(new_expression.as_str(), re_operators);
        result += calculate_subexpression(numbers, operators);
    }

    println!("Result: {}", result);

}

fn calculate_subexpression(
    numbers: Vec<String>, 
    operators: Vec<String>
) -> i32 {

    let first_number: String = operators[0].to_owned() + numbers[0].as_str();
    let mut subresult: i32 = first_number.parse::<i32>().unwrap();

    for i in 1..operators.len() {
        let number = numbers[i].parse::<i32>().unwrap();
        let action_result = match_numbers(subresult, number, &operators[i]);
        subresult = action_result;
    }

    return subresult;

}

fn match_numbers(
    first_number: i32,
    second_number: i32, 
    operator: &String
) -> i32 {

    match operator.as_str() {
        "+" => return first_number + second_number,
        "-" => return first_number - second_number,
        "*" => return first_number * second_number,
        "/" => return first_number / second_number,
        _ => todo!(),
    }

}

// Splits a string with a given regular expression
fn re_split(
    string: &str, 
    expression: &str
) -> Vec<String> {

    let re = Regex::new(expression).unwrap();

    let result = re.captures_iter(&string)
        .map(|c| c.get(0).unwrap().as_str().to_string())
        .collect::<Vec<_>>();

    return result;
            
}

// Removes matching parts of a string
fn re_remove(
    string: &str,
    expression: &str
) -> String {

    let re = Regex::new(expression).unwrap();
    let result = re.replace_all(string, "").to_string();

    return result;
}