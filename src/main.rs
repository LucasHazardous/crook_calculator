use std::io;

enum Element {
    Number(f64),
    Operation(char)
}

fn main() {
    println!("Welcome to Level 1 Crook Calculator.");

    let tiers = vec![vec!['^'], vec!['*', '/'], vec!['+', '-']];

    'main_loop:
    loop {
        println!("Enter new expression:");
        
        // Load raw input
        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            continue 'main_loop;
        }

        // Validate and load into elements
        let mut elements: Vec<Element> = Vec::new();
        let mut next_sign = false;

        for element in input.split_whitespace() {
            if next_sign {
                let mut chars = element.chars();
                let first_char = chars.next().unwrap();

                if chars.count() == 0 && tiers_contain_operator(&tiers, &first_char) {
                    elements.push(Element::Operation(first_char));
                } else {
                    continue 'main_loop;
                }
                next_sign = false;
            } else {
                if let Ok(num) = element.parse::<f64>() {
                    elements.push(Element::Number(num));
                } else {
                    continue 'main_loop;
                }

                next_sign = true;
            }
        }
        if !next_sign {
            continue 'main_loop;
        }

        // Compute tiers sequentially
        for tier in &tiers {
            let mut i = 0;
            while i < elements.len() {

                if let Element::Operation(operation) = elements[i] {
                    if tier.contains(&operation) {

                        if let Element::Number(left_side) = elements[i-1] {
                            if let Element::Number(right_side) = elements[i+1] {

                                elements[i+1] = Element::Number(compute_operation(&operation, left_side, right_side));

                                elements.remove(i);
                                elements.remove(i-1);
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

        if let Element::Number(res) = elements[0] {
            println!("Result: {res}");
        }
    }
}


fn tiers_contain_operator(tiers: &Vec<Vec<char>>, operator: &char) -> bool {
    for tier in tiers {
        if tier.contains(operator) {
            return true;
        }
    }
    return false;
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