use crate::calculator::Calculator;
use crate::gui_helpers::{
    create_number_callback, create_operator_callback, setup_button, setup_input,
};
use crate::operators::Operator;
use fltk::{app, prelude::*, window::Window};
use std::cell::RefCell;
use std::rc::Rc;

static WINDOW_WIDTH: i32 = 300;
static WINDOW_HEIGHT: i32 = 300;
static OPERATOR_WIDTH: i32 = 40;
static OPERATOR_HEIGHT: i32 = 20;
static INPUT_WIDTH: i32 = 200;
static INPUT_HEIGHT: i32 = 30;

fn initialize_gui() {
    let calculator = Rc::new(RefCell::new(Calculator::new()));
    let input = Rc::new(RefCell::new(setup_input(
        100,
        20,
        INPUT_WIDTH,
        INPUT_HEIGHT,
    )));

    let operations = [
        ("+", Operator::Add, 160, 200),
        ("-", Operator::Subtract, 210, 200),
        ("*", Operator::Multiply, 160, 230),
        ("/", Operator::Divide, 210, 230),
        ("=", Operator::Add, 160, 260),
        ("C", Operator::Clear, 210, 260),
    ];

    let numbers = [
        ("1", 1, 10, 200),
        ("2", 2, 60, 200),
        ("3", 3, 110, 200),
        ("4", 4, 10, 230),
        ("5", 5, 60, 230),
        ("6", 6, 110, 230),
        ("7", 7, 10, 260),
        ("8", 8, 60, 260),
        ("9", 9, 110, 260),
        ("0", 0, 60, 290),
    ];

    let mut buttons = Vec::new();
    for (text, op, x, y) in operations.iter() {
        let button = setup_button(
            *x,
            *y,
            OPERATOR_WIDTH,
            OPERATOR_HEIGHT,
            text,
            create_operator_callback(*op, calculator.clone(), input.clone()),
        );
        buttons.push(button);
    }

    for (text, number, x, y) in numbers.iter() {
        let button = setup_button(
            *x,
            *y,
            OPERATOR_WIDTH,
            OPERATOR_HEIGHT,
            text,
            create_number_callback(*number, input.clone()),
        );
        buttons.push(button);
    }

    let mut window = Window::default()
        .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_pos(100, 100)
        .with_label("Hello from rust");
    for button in &buttons {
        window.add(button);
    }
    window.add(&*input.borrow());
    window.end();
    window.show();
}

pub fn run() {
    initialize_gui();
    app::App::default().run().unwrap();
}
