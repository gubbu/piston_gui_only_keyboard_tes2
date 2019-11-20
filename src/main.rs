extern crate piston_window;

use piston_window::*;
mod text_render;

mod basic_mouse_ui;

#[derive(PartialEq, Eq, Hash)]
enum ExampleUiElements {
    ADD,
    SUBTRACT,
    LABEL,
    MUL,
    Set0
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .build()
        .unwrap();
    let mut test_gui = basic_mouse_ui::VerticalMenu::new(3, 2, [2.0;2], [0.0, 1.0, 1.0, 1.0]);

    let mut count = 0i128;
    test_gui.add_text(
        ExampleUiElements::LABEL,
        format!("count: {}", count),
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    );
    test_gui.add_text(
        ExampleUiElements::ADD,
        String::from("ADD +"),
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    );
    test_gui.add_text(
        ExampleUiElements::SUBTRACT,
        String::from("SUB -"),
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    );

    test_gui.add_text(
        ExampleUiElements::MUL,
        String::from("*=self [overflow]"),
            //255, 208, 0
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 0.5, 0.0, 1.0]
    );

    test_gui.add_text(
        ExampleUiElements::Set0,
        String::from("reset\nset back to 0."),
        //255, 208, 0
        [1.0, 0.0, 0.0, 1.0],
        [1.0, 255.0/208.0, 0.0, 1.0]
    );

    while let Some(e) = window.next() {
        if let Some(args) = e.button_args() {
            if args.state == piston_window::ButtonState::Release {
                if args.button == piston_window::Button::Keyboard(piston_window::Key::Up) {
                    test_gui.on_up();
                } else if args.button == piston_window::Button::Keyboard(piston_window::Key::Down) {
                    test_gui.on_down();
                } else if args.button == piston_window::Button::Keyboard(piston_window::Key::Return)
                {
                    if let Some(test) = test_gui.on_enter() {
                        match test {
                            ExampleUiElements::ADD => count += 1,
                            ExampleUiElements::SUBTRACT => count -= 1,
                            ExampleUiElements::Set0 => count= 0,
                            ExampleUiElements::MUL=> count = count.wrapping_mul(count),
                            _ => {
                                continue;
                            }
                        }
                        println!("calling update");
                        test_gui.set_text(&ExampleUiElements::LABEL, format!("{}", count));
                    }
                }
            }
        }
        window.draw_2d(&e, |c, g, _| {
            test_gui.render(64.0, 64.0, 10.0, c.transform, g);
            clear([1.0, 0.5, 0.5, 1.0], g);
            //rectangle([0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 80.0, 80.0], c.transform, g);
            text_render::text_render::draw_filled_string(
                &String::from(
                    "\
                Hello\nWorld\n?!\n1/3 = 0.3333... (?)
                ",
                ),
                5.0,
                5.0,
                2.0,
                2.0,
                [0.0, 0.0, 1.0, 1.0],
                None,
                c.transform,
                g,
            );
        });
    }
}
