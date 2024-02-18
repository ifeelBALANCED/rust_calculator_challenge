#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use rust_calculator_challenge::expression_evaluator::{Evaluatable, ExpressionEvaluator};

fn main() {
    let expression = String::from("2 * 3 + 4");
    let expression_evaluator = ExpressionEvaluator::new(expression);
    println!("{}", expression_evaluator.evaluate());
}
