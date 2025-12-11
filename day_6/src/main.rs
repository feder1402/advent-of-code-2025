#[derive(Debug, Clone)]
struct Problem {
    arguments: Vec<String>,
    operation: Option<String>,
}

fn main() {
    // Read the input
    let input = include_str!("../sample_input.txt");
    let lines = input.lines();

    // Load the input into a problem collection
    let mut problem_list: Vec<Problem> = Vec::new();
    for line in lines {
        let elements: Vec<String> = line
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if elements.len() == 0 {
            panic!("Empty line");
        }

        println!("Line: \"{line}\" converted to elements: {:?}", elements);

        if elements[0] == "+" || elements[0] == "*" {
            for (i, element) in elements.iter().enumerate() {
                problem_list[i].operation = Some(element.to_string());
            }
            break;
        }

        // Add the arguments
        if problem_list.len() == 0 {
            for element in elements {
                let mut arguments = Vec::new();
                arguments.push(element);
                problem_list.push(Problem {
                    arguments,
                    operation: None,
                });
            }
        } else {
            for (i, element) in elements.iter().enumerate() {
                problem_list[i]
                    .arguments
                    .push(element.to_string());
            }
        }
    }

    for problem in problem_list {
        println!("Problem: {:?}", problem);
    }

    // let total = solve_problems_part2(problem_list);

    // println!("Total: {}", total);
}

// fn solve_problems_part2(problem_list: Vec<Problem>) -> i64 {
//     // Before solving the problems, we need to convert the arguments to 'cephalopod math' that is written right-to-left in columns
//     let mut problem_list_2 = Vec::new();

//     for problem in problem_list {
//         let mut new_arguments = Vec::new();
//         let mut value = 0;
//         let mut divider = 10;
//         loop {
//             for num in &problem.arguments {
//                 let new_num = (num / divider) * divider;
//                 value += num - new_num;
//             }
//             divider *= 10;

//             // Need to find final condition
//             if value == 0 {
//                 break;
//             }
//         }

//         problem_list_2.push(Problem {
//             arguments: new_arguments,
//             operation: problem.operation,
//         });
//     }

//     // solve_problems_part1(problem_list_2)
//     0
// }

// fn solve_problems_part1(problem_list: Vec<Problem>) -> i64 {
//     // Once we got all problems, solve them
//     let mut total = 0;
//     for (i, problem) in problem_list.iter().enumerate() {
//         let operation = problem.operation.clone().unwrap();
//         let mut result = if operation == Ops::Add { 0 } else { 1 };
//         for argument in problem.arguments.clone() {
//             if operation == Ops::Add {
//                 result += argument;
//             } else {
//                 result *= argument;
//             }
//         }
//         println!(
//             "Problem: {}: {} = {}",
//             i,
//             problem
//                 .arguments
//                 .iter()
//                 .map(|v| v.to_string())
//                 .collect::<Vec<String>>()
//                 .join(if operation == Ops::Add { " + " } else { " * " }),
//             result
//         );
//         total += result;
//     }

//     total
// }
