use crate::{Element, TIERS};

pub fn convert_input_to_equation(input: &str) -> Option<Vec<Element>> {
    let mut elements: Vec<Element> = Vec::new();
    let mut next_sign = false;
    let mut brackets = 0;

    for element in input.split_whitespace() {
        if next_sign {
            let (first_char, remaining_char_count) = first_char_and_remaining_count(element);

            if remaining_char_count == 0 && tiers_contain_operator(&first_char) {
                elements.push(Element::Operation(first_char));
                next_sign = false;
            } else if remaining_char_count == 0 && first_char == ')' {
                brackets -= 1;
                if brackets < 0 {
                    return None;
                }
                elements.push(Element::CloseBracket);
            } else {
                return None;
            }
        } else {
            let (first_char, remaining_char_count) = first_char_and_remaining_count(element);

            if let Ok(num) = element.parse::<f64>() {
                elements.push(Element::Number(num));
                next_sign = true;
            } else if remaining_char_count == 0 && first_char == '(' {
                brackets += 1;
                elements.push(Element::OpenBracket);
            } else {
                return None;
            }
        }
    }
    if !next_sign || brackets > 0 {
        return None;
    }

    Some(elements)
}

fn first_char_and_remaining_count(element: &str) -> (char, usize) {
    let mut chars = element.chars();
    let first_char = chars.next().unwrap();
    let remaining_char_count = chars.count();

    (first_char, remaining_char_count)
}

fn tiers_contain_operator(operator: &char) -> bool {
    TIERS
        .iter()
        .flatten()
        .collect::<Vec<&char>>()
        .contains(&operator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Element::*;

    #[test]
    fn first_char_and_remaining_count_test() {
        assert_eq!(first_char_and_remaining_count("abc"), ('a', 2));
    }

    #[test]
    fn tiers_contain_operator_test() {
        TIERS
            .iter()
            .flatten()
            .for_each(|operator| {
                assert!(tiers_contain_operator(operator))
            })
    }

    #[test]
    fn convert_input_to_equation_test() {
        assert_eq!(convert_input_to_equation("2 * (  1 +  1 )  "), 
        Some(vec![Number(2.), Operation('*'), OpenBracket, Number(1.),
        Operation('+'), Number(1.), CloseBracket]));
    }

    #[test]
    fn convert_input_to_equation_invalid_bracket_test() {
        assert_eq!(convert_input_to_equation("2 * ( ) 1 +  1 )"), None);
    }

    #[test]
    fn convert_input_to_equation_invalid_test() {
        assert_eq!(convert_input_to_equation("2 * (  1 $ +1 )"), None);
    }

    #[test]
    fn convert_input_to_equation_empty_test() {
        assert_eq!(convert_input_to_equation("   \n  "), None);
    }
}