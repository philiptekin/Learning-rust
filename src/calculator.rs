pub struct Calculator {
    current_value: i32,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { current_value: 0 }
    }

    pub fn get_current_value(&self) -> i32 {
        self.current_value
    }

    pub fn reset(&mut self) {
        self.current_value = 0;
    }

    pub fn add(&mut self, value: i32) {
        self.current_value += value;
    }

    pub fn subtract(&mut self, value: i32) {
        self.current_value -= value;
    }

    pub fn multiply(&mut self, value: i32) {
        self.current_value *= value;
    }

    pub fn divide(&mut self, value: i32) -> Result<(), String> {
        if value == 0 {
            Err("Cannot divide by zero".to_string())
        } else {
            self.current_value /= value;
            Ok(())
        }
    }
}
