use fltk::{app, button, frame, input, prelude::*, window};
use fltk_flex::Flex;
use fltk_theme::{ThemeType, WidgetTheme};

fn main() {
    let app = app::App::default();
    let theme = WidgetTheme::new(ThemeType::Aero);
    theme.apply();

    let mut win = window::Window::default().with_size(400, 300);
    let flex = Flex::default()
        .with_size(200, 100)
        .center_of_parent()
        .column();

    let mut inp = input::Input::default();
    let mut frame = frame::Frame::default();

    let mut btn = button::Button::new(160, 200, 80, 30, "Click");

    flex.end();
    win.end();
    win.show();

    btn.set_callback(move |b| {
        frame.set_label(&inp.value());
    });

    app.run().unwrap();
}

// fn theme_botton(btn: &mut button::Button) {
//     btn.clear_visible_focus();
//     btn.set_color(Color::from_rgb(255, 0, 0).inactive());
//     btn.set_selection_color(Color::from_rgb(255, 0, 0).darker());
// }
