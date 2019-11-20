extern crate piston_window;

use piston_window::*;
mod text_render;

mod basic_keyboard_ui;

//these are the different elements that are visible inside our demo. the derives are compiler enforced, if used as identifiers in a basic_keyboard_ui::VerticalMenu
#[derive(PartialEq, Eq, Hash)]
enum ExampleUiElements {
    ADD,
    SUBTRACT,
    LABEL,
    MUL,
    Set0,
}

fn main() {
    //creating the main window
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .build()
        .unwrap();
    //creating the vertical Menu, configuring, how many elements are visible at a time (view_up = 3 + view_down=2)=5, the size of the bitmap font pixels, the background of the currently selected element.
    //now: make the background invisible: [0.0;4]
    let mut test_gui = basic_keyboard_ui::VerticalMenu::new(3, 2, [2.0; 2], [0.0;4]);
    //the count number has to be stored over the whole runtime
    //it can be modified by this interactive demo application. Its the demo applications state.
    let mut count = 0i128;
    //now adding all the elements and their identifiers: (the enum variants declared above).
    test_gui.add_text(
        ExampleUiElements::LABEL,
        format!("count: {}", count),
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    );
    //the interactive buttons: the interactivity logic is declared in the mainloop, if certain events are detected.
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
        String::from("*=self [wrapping]"),
        //255, 208, 0
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 0.5, 0.0, 1.0],
    );

    test_gui.add_text(
        ExampleUiElements::Set0,
        String::from("reset\nset back to 0."),
        //255, 208, 0
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 255.0 / 208.0, 0.0, 1.0],
    );

    while let Some(e) = window.next() {
        //evaluating all polled events while in runtime
        if let Some(args) = e.button_args() {
            if args.state == piston_window::ButtonState::Release {
                //the application reacts and changes state, only if certain keys are pressed:
                if args.button == piston_window::Button::Keyboard(piston_window::Key::Up) {
                    test_gui.on_up();
                } else if args.button == piston_window::Button::Keyboard(piston_window::Key::Down) {
                    test_gui.on_down();
                } else if args.button == piston_window::Button::Keyboard(piston_window::Key::Return)
                {
                    //if Return is pressed: the resulting currently selected identifier is thrown
                    // the identifier is evaluated and the counter variable changed accordingly
                    if let Some(test) = test_gui.on_enter() {
                        match test {
                            ExampleUiElements::ADD => count += 1,
                            ExampleUiElements::SUBTRACT => count -= 1,
                            ExampleUiElements::Set0 => count = 0,
                            ExampleUiElements::MUL => count = count.wrapping_mul(count),
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
            //rendering the ui: at x=y=64.0 and a gap of 10.0 between the elements
            test_gui.render_static(64.0, 64.0, 10.0, c.transform, g, [1.0; 4]);
            //background rgba
            clear([1.0, 0.5, 0.5, 1.0], g);
            //rendering some static text in the background to demonstrate text_render functionality
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
