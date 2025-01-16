use fltk::{
    app::App,
    enums::CallbackTrigger,
    frame::Frame,
    group,
    input::{self},
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
    window::Window,
};

fn main() {
    let app = App::default();
    let mut win = Window::new(200, 200, 150, 250, "");
    let pack = group::Pack::new(0, 0, 100, 250, "");
    let _frame1 = Frame::new(0, 0, 0, 50, "Celcius");
    let mut inp1 = input::FloatInput::new(0, 0, 50, 30, "");
    let _frame2 = Frame::new(0, 0, 0, 50, "Фаренгейт");
    let mut inp2 = input::FloatInput::new(0, 0, 50, 30, "");
    pack.end();
    win.end();
    win.show();

    inp1.set_value(&format!("{}", 0.0));
    inp2.set_value(&format!("{}", 32.0));

    inp1.set_trigger(CallbackTrigger::Changed);
    inp2.set_trigger(CallbackTrigger::Changed);

    app.run().unwrap();
}

fn c_to_f(val: f64) -> f64 {
    (val * 9.0 / 5.0) + 32.0
}

fn f_to_c(val: f64) -> f64 {
    (val - 32.0) * 5.0 / 9.0
}
