use crate::calculator::Calculator;

#[derive(Copy, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Clear,
}

impl Operator {
    pub fn apply(&self, calc: &mut Calculator, value: i32) {
        match self {
            Operator::Add => calc.add(value),
            Operator::Subtract => calc.subtract(value),
            Operator::Multiply => calc.multiply(value),
            Operator::Divide => {
                if let Err(e) = calc.divide(value) {
                    println!("Error: {}", e);
                }
            }
            Operator::Clear => calc.reset(),
        }
    }
}
