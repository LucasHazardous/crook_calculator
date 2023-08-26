use crate::{Element, TIERS};

pub fn compute_equation(elements: &mut Vec<Element>) {
    let mut current_pos = 0;
    let mut stack: Vec<usize> = Vec::new();

    while current_pos < elements.len() {
        match elements[current_pos] {
            Element::OpenBracket => stack.push(current_pos),
            Element::CloseBracket => {
                current_pos -= compute_equation_without_brackets(
                    elements,
                    stack.pop().unwrap(),
                    current_pos
                );
            },
            _ => ()
        }
        current_pos += 1;
    }

    let elements_len = elements.len();
    compute_equation_without_brackets(elements, 0, elements_len-1);
}

fn compute_equation_without_brackets(elements: &mut Vec<Element>, from: usize, to: usize) -> usize {
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

    for tier in TIERS {
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