mod input_processing;
mod equation_computation;
mod element;

use input_processing::convert_input_to_equation;
use equation_computation::compute_equation;
use element::Element;

const TIERS: [[char; 2]; 3] = [['^', '^'], ['*', '/'], ['+', '-']];

pub fn compute(input: String) -> Option<f64> {
    Some(compute_equation(convert_input_to_equation(input)?))
}