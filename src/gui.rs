use crate::calculator;
use fltk::{app, button::Button, input::Input, prelude::*, window::Window};

static WINDOW_WIDTH: i32 = 300;
static WINDOW_HEIGHT: i32 = 300;

static OPERATOR_WIDTH: i32 = 40;
static OPERATOR_HEIGHT: i32 = 20;

fn initialize_gui() {
    let button = setup_button(160, 200, OPERATOR_WIDTH, OPERATOR_HEIGHT, "+");

    let input = setup_input();

    let mut window = Window::new(100, 100, WINDOW_WIDTH, WINDOW_HEIGHT, "Hello from rust");
    window.add(&button);
    window.add(&input);
    window.end();
    window.show();
}

fn setup_button(x: i32, y: i32, width: i32, height: i32, text: &str) -> Button {
    let mut button = Button::new(x, y, width, height, text);
    button.set_callback(move |_| {
        let result = calculator::addition(2, 3);
        println!("Result of addition: {}", result);
    });
    button
}

fn setup_input() -> Input {
    let input = Input::new(100, 20, 200, 30, "");
    input
}

pub fn run() {
    initialize_gui();
    app::App::default().run().unwrap();
}
