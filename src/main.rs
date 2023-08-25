use std::io;
use crook_calculator::*;

fn main() {
    println!("Welcome to Level 1 Crook Calculator.");

    'main_loop:
    loop {
        println!("Enter new expression:");
        
        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            continue 'main_loop;
        }

        if let ConversionResult::Result(mut elements, bracket_pair_count) = convert_input_to_equation(input) {
            compute_equation(&mut elements, bracket_pair_count);
        
            if let Element::Number(res) = elements[0] {
                println!("Result: {res}");
            }
        }
    }
}
