//! # Crook calculator
//! `crook_calculator` provides one function to compute 
//! mathematical equations from string slices.

mod input_processing;
mod equation_computation;
mod element;

use input_processing::convert_input_to_equation;
use equation_computation::compute_equation;
use element::Element;

const TIERS: [[char; 2]; 3] = [['^', '^'], ['*', '/'], ['+', '-']];

/// Computes mathematical equation from a string slice.
/// The input is required to contain supported operators and valid *f64* numbers,
/// separated by any amount of different whitespace characters.
/// 
/// # Examples
/// 
/// ```
/// assert_eq!(crook_calculator::compute("2.5 - .5").unwrap(), 2.);
/// ```
/// 
/// ```
/// assert_eq!(crook_calculator::compute("2 +  2 *  3 \n ^ \t  2 - ( 2 + -1 )").unwrap(), 19.);
/// ```
/// 
/// ```
/// assert_eq!(crook_calculator::compute("( 1 + ( 1 )"), None);
/// ```
/// 
/// ```
/// assert_eq!(crook_calculator::compute("5 + a + 2"), None);
/// ```
/// 
/// # Supported operators
/// 
/// * addtion **+** and subtraction **-**
/// * multiplication **\*** and division **\/**, division by 0 results in *inf* value
/// * exponentiation **^**
/// * brackets **(**  **)**, for changing operation order
pub fn compute(input: &str) -> Option<f64> {
    Some(compute_equation(convert_input_to_equation(input)?))
}