#![warn(clippy::all)]
#![warn(clippy::pedantic)]
mod expression_evaluator;
use expression_evaluator::{Evaluatable, ExpressionEvaluator};

fn main() {
    let expression = String::from("2 * 3 + 4");
    let expression_evaluator = ExpressionEvaluator::new(expression);
    println!("{}", expression_evaluator.evaluate());
}
