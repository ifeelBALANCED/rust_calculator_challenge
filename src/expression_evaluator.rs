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
