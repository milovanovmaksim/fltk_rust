use fltk::{
    app,
    button::{self, Button},
    enums::{CallbackTrigger, Event},
    frame, input,
    prelude::*,
    widget_extends, window,
};
use fltk_flex::Flex;
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};

struct MyButton {
    btn: Button,
}

impl MyButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut btn = button::Button::new(x, y, w, h, None).with_label(label);
        btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
        btn.handle(|b, ev| match ev {
            Event::Enter => {
                b.set_frame(widget_themes::OS_BUTTON_UP_FRAME);
                b.redraw();
                true
            }
            Event::Leave => {
                b.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
                b.redraw();
                true
            }

            _ => false,
        });
        Self { btn }
    }

    pub fn default() -> Self {
        Self::new(0, 0, 0, 0, "")
    }
}

widget_extends!(MyButton, button::Button, btn);

fn main() {
    let app = app::App::default();
    let theme = WidgetTheme::new(ThemeType::Blue);
    theme.apply();

    let mut win = window::Window::default().with_size(400, 300);
    let flex = Flex::default()
        .with_size(200, 100)
        .center_of_parent()
        .column();

    let mut inp = input::Input::default();
    inp.set_trigger(CallbackTrigger::Changed);
    let mut frame = frame::Frame::default();

    let mut btn = MyButton::default().with_label("Click");

    flex.end();
    win.end();
    win.show();
    inp.set_callback({
        let mut frame = frame.clone();
        move |i| {
            frame.set_label(&i.value());
        }
    });

    app.run().unwrap();
}

// fn theme_botton(btn: &mut button::Button) {
//     btn.clear_visible_focus();
//     btn.set_color(Color::from_rgb(255, 0, 0).inactive());
//     btn.set_selection_color(Color::from_rgb(255, 0, 0).darker());
// }
