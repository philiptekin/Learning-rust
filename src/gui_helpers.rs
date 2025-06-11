use crate::calculator::Calculator;
use crate::operators::Operator;
use fltk::{button::Button, input::Input, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;

pub fn setup_button<F>(x: i32, y: i32, width: i32, height: i32, text: &str, callback: F) -> Button
where
    F: 'static + FnMut(&mut Button),
{
    let mut button = Button::new(x, y, width, height, text);
    button.set_callback(callback);
    button
}

pub fn setup_input(x: i32, y: i32, width: i32, height: i32) -> Input {
    Input::new(x, y, width, height, "")
}

pub fn create_operator_callback(
    operator: Operator,
    calculator: Rc<RefCell<Calculator>>,
    input: Rc<RefCell<Input>>,
) -> impl 'static + FnMut(&mut Button) {
    move |_| {
        if let Ok(num) = input.borrow().value().trim().parse::<i32>() {
            operator.apply(&mut calculator.borrow_mut(), num);
            println!("Result: {}", calculator.borrow().get_current_value());
        } else {
            println!("Invalid number.");
        }
    }
}

pub fn create_number_callback(
    number: i32,
    input: Rc<RefCell<Input>>,
) -> impl 'static + FnMut(&mut Button) {
    move |_| {
        let current_value = input.borrow().value();
        let new_value = if current_value.is_empty() {
            number.to_string()
        } else {
            format!("{}{}", current_value, number)
        };
        input.borrow_mut().set_value(&new_value);
    }
}
