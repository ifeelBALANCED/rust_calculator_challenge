#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub struct ExpressionEvaluator {
    pub expression: String,
}

pub trait Evaluatable {
    fn new(expression: String) -> Self;
    fn evaluate(&self) -> i32;
}

impl Evaluatable for ExpressionEvaluator {
    fn new(expression: String) -> Self {
        Self { expression }
    }

    fn evaluate(&self) -> i32 {
        0
    }
}

fn main() {
    let expression = String::from("2 * 3 + 4");
    let expression_evaluator = ExpressionEvaluator::new(expression);
    println!("{}", expression_evaluator.evaluate());
}
