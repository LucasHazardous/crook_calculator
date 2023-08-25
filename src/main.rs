use std::io;

enum Element {
    Number(f64),
    Operation(char),
    OpenBracket,
    CloseBracket
}

fn main() {
    println!("Welcome to Level 1 Crook Calculator.");

    let tiers = vec![vec!['^'], vec!['*', '/'], vec!['+', '-']];

    'main_loop:
    loop {
        println!("Enter new expression:");
        
        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            continue 'main_loop;
        }

        let mut elements: Vec<Element> = Vec::new();
        let mut next_sign = false;
        let mut brackets = 0;
        let mut bracket_iter = 0;

        for element in input.split_whitespace() {
            if next_sign {
                let (first_char, remaining_char_count) = first_char_and_remaining_count(element);

                if remaining_char_count == 0 && tiers_contain_operator(&tiers, &first_char) {
                    elements.push(Element::Operation(first_char));
                    next_sign = false;
                } else if remaining_char_count == 0 && first_char == ')' {
                    brackets -= 1;
                    if brackets < 0 {
                        continue 'main_loop;
                    }
                    elements.push(Element::CloseBracket);
                } else {
                    continue 'main_loop;
                }
            } else {
                let (first_char, remaining_char_count) = first_char_and_remaining_count(element);

                if let Ok(num) = element.parse::<f64>() {
                    elements.push(Element::Number(num));
                    next_sign = true;
                } else if remaining_char_count == 0 && first_char == '(' {
                    brackets += 1;
                    bracket_iter += 1;
                    elements.push(Element::OpenBracket);
                } else {
                    continue 'main_loop;
                }
            }
        }
        if !next_sign || brackets > 0 {
            continue 'main_loop;
        }

        compute_equation(&tiers, &mut elements, bracket_iter);
        
        if let Element::Number(res) = elements[0] {
            println!("Result: {res}");
        }
    }
}

fn first_char_and_remaining_count(element: &str) -> (char, usize) {
    let mut chars = element.chars();
    let first_char = chars.next().unwrap();
    let remaining_char_count = chars.count();

    (first_char, remaining_char_count)
}

fn tiers_contain_operator(tiers: &Vec<Vec<char>>, operator: &char) -> bool {
    for tier in tiers {
        if tier.contains(operator) {
            return true;
        }
    }
    return false;
}

fn compute_equation(tiers: &Vec<Vec<char>>, elements: &mut Vec<Element>, bracket_count: usize) {
    let mut current_pos = 0;
    let mut stack: Vec<usize> = Vec::new();
    let mut bracket_count = bracket_count;

    while bracket_count > 0 && current_pos < elements.len() {
        match elements[current_pos] {
            Element::OpenBracket => stack.push(current_pos),
            Element::CloseBracket => {
                current_pos -= compute_equation_without_brackets(
                    &tiers,
                    elements,
                    stack.pop().unwrap(),
                    current_pos
                );
                bracket_count -= 1;
            },
            _ => ()
        }
        current_pos += 1;
    }

    let elements_len = elements.len();
    compute_equation_without_brackets(&tiers, elements, 0, elements_len-1);
}

fn compute_equation_without_brackets(tiers: &Vec<Vec<char>>, elements: &mut Vec<Element>, from: usize, to: usize) -> usize {
    let mut reduced: usize = 0;
    
    let mut to = to;
    let mut from = from;
    if let Element::OpenBracket = elements[from] {
        elements.remove(from);
        to -= 1;
        elements.remove(to);
        from += 1;

        reduced += 2;
    }

    for tier in tiers {
        let mut i = from;
        while i < to {

            if let Element::Operation(operation) = elements[i] {
                if tier.contains(&operation) {

                    if let Element::Number(left_side) = elements[i-1] {
                        if let Element::Number(right_side) = elements[i+1] {

                            elements[i+1] = Element::Number(compute_operation(&operation, left_side, right_side));

                            elements.remove(i);
                            elements.remove(i-1);
                            to -= 2;
                            reduced += 2;
                        }
                    }
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    reduced
}

fn compute_operation(operation: &char, left_side: f64, right_side: f64) -> f64 {
    match operation {
        '^' => left_side.powf(right_side),
        '*' => left_side*right_side,
        '/' => if right_side != 0. { left_side/right_side } else { f64::INFINITY }
        '+' => left_side+right_side,
        '-' => left_side-right_side,
        _ => 0.
    }
}