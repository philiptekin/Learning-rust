use crate::calculator::Calculator;
use fltk::{app, button::Button, input::Input, prelude::*, window::Window};
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

    let button = setup_button(
        160,
        200,
        OPERATOR_WIDTH,
        OPERATOR_HEIGHT,
        "+",
        calculator.clone(),
    );

    let input = setup_input(100, 20, INPUT_WIDTH, INPUT_HEIGHT);

    let mut window = Window::new(100, 100, WINDOW_WIDTH, WINDOW_HEIGHT, "Hello from rust");
    window.add(&button);
    window.add(&input);
    window.end();
    window.show();
}

fn setup_button(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    text: &str,
    calculator: Rc<RefCell<Calculator>>,
) -> Button {
    let mut button = Button::new(x, y, width, height, text);
    button.set_callback(move |_| {
        calculator.borrow_mut().add(3);
        println!(
            "Result of addition: {}",
            calculator.borrow_mut().get_current_value()
        );
    });
    button
}

fn setup_input(x: i32, y: i32, width: i32, height: i32) -> Input {
    let input = Input::new(x, y, width, height, "");
    input
}

pub fn run() {
    initialize_gui();
    app::App::default().run().unwrap();
}
