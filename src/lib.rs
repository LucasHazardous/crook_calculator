mod input_processing;
mod equation_computation;

pub use input_processing::convert_input_to_equation;
pub use equation_computation::compute_equation;

pub enum Element {
    Number(f64),
    Operation(char),
    OpenBracket,
    CloseBracket
}

pub enum ConversionResult {
    Result(Vec<Element>, usize),
    None
}

const TIERS: [[char; 2]; 3] = [['^', '^'], ['*', '/'], ['+', '-']];
