use crate::{Element, ConversionResult, TIERS};

pub fn convert_input_to_equation(input: String) -> ConversionResult {
    let mut elements: Vec<Element> = Vec::new();
    let mut next_sign = false;
    let mut brackets = 0;
    let mut bracket_iter = 0;

    for element in input.split_whitespace() {
        if next_sign {
            let (first_char, remaining_char_count) = first_char_and_remaining_count(element);

            if remaining_char_count == 0 && tiers_contain_operator(&first_char) {
                elements.push(Element::Operation(first_char));
                next_sign = false;
            } else if remaining_char_count == 0 && first_char == ')' {
                brackets -= 1;
                if brackets < 0 {
                    return ConversionResult::None;
                }
                elements.push(Element::CloseBracket);
            } else {
                return ConversionResult::None;
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
                return ConversionResult::None;
            }
        }
    }
    if !next_sign || brackets > 0 {
        return ConversionResult::None;
    }

    ConversionResult::Result(elements, bracket_iter)
}

fn first_char_and_remaining_count(element: &str) -> (char, usize) {
    let mut chars = element.chars();
    let first_char = chars.next().unwrap();
    let remaining_char_count = chars.count();

    (first_char, remaining_char_count)
}

fn tiers_contain_operator(operator: &char) -> bool {
    for tier in &TIERS {
        if tier.contains(operator) {
            return true;
        }
    }
    return false;
}