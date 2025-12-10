use std::cmp::PartialEq;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Ops {
    Add,
    Mul,
}

struct Problem {
    arguments: Vec<i64>,
    operation: Option<Ops>,
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    let lines = input.lines();

    let mut problem_list: Vec<Problem> = Vec::new();
    for line in lines {
        let elements = line.split_whitespace().map(|x| x.trim()).collect::<Vec<&str>>();

        if elements.len() == 0 {
            panic!("Empty line");
        }

        if elements[0] == "+" || elements[0] == "*" {
            for (i, element) in elements.iter().enumerate() {
                let operation = match element {
                    &"+" => Ops::Add,
                    &"*" => Ops::Mul,
                    _ => panic!("Unknown operation"),
                };
                problem_list[i].operation = Some(operation);
            }
            break;
        }

        if problem_list.len() == 0 {
            for element in elements {
                let mut arguments = Vec::new();
                arguments.push(element.parse::<i64>().unwrap());
                problem_list.push(Problem { arguments: arguments, operation: None});
            }
        } else {
            for (i, element) in elements.iter().enumerate() {
                problem_list[i].arguments.push(element.parse::<i64>().unwrap());
            }
        }
    }

    let mut total = 0;
    for (i, problem) in problem_list.iter().enumerate() {
        let operation = problem.operation.clone().unwrap();
        let mut result = if operation == Ops::Add { 0 } else { 1 };
        for argument in problem.arguments.clone() {
            if operation == Ops::Add {
                result += argument;
            } else {
                result *= argument;
            }
        }
        println!("Problem: {}: {} = {}", i, problem.arguments.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(if operation == Ops::Add {" + "} else {" * "}), result);
        total += result;
    }

    println!("Total: {}", total);

    //println!("Read {} lines", lines.by_ref().count());

    // if last.is_some() {
    //     println!("Last line: {}", last.unwrap());
    // } else {
    //     println!("No lines");
    // }


}
