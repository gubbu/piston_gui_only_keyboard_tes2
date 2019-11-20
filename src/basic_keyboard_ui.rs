//every UI Element has to have a constant height, so that by whole number division the mouse_over
// element can be found by O(1)

pub struct VerticalMenu<T: std::cmp::Eq + std::hash::Hash> {
    pub selected: i64,
    pub view_up: i64,
    pub view_down: i64,
    pub block_width: f64,
    pub block_height: f64,
    //explaining the tuple:
    // the text String, its height as f64, its color rgba information: [0]: the rgba color, if selected
    // [1]: the color, if selected.
    // the key of type T in the hashmap is the ui-elements identifier that can bes used, to:
    // a) identify the source of a event, make according changes to the underlying app state.
    // b) modify the specific already added element, if its identifier is known.
    // c) remove/delete the specific element, from the ui.
    pub options: std::collections::HashMap<T, (String, f64, [[f32; 4]; 2])>,
    pub selected_background: [f32; 4],
}

impl<T: std::cmp::Eq + std::hash::Hash> VerticalMenu<T> {
    pub fn new(
        view_up: i64,
        view_down: i64,
        text_pixel_size: [f64; 2],
        selected_background_rgba: [f32; 4],
    ) -> Self {
        return VerticalMenu {
            selected: 0,
            view_up,
            view_down,
            block_width: text_pixel_size[0],
            block_height: text_pixel_size[1],
            options: Default::default(),
            selected_background: selected_background_rgba,
        };
    }

    ///render the menu: render view_up menu entry above and self.view_down-1 below the currently selected option
    pub fn render(
        &self,
        x: f64,
        mut y: f64,
        gap: f64,
        transform: [[f64; 3]; 2],
        graphics: &mut impl piston_window::Graphics,
        line_color: [f32; 4],
    ) {
        let i_length = self.options.len() as i64;
        let old_y = y;
        for i in self.selected - self.view_up..self.selected + self.view_down {
            let pythons_index = ((i.abs() + i) / 2) % i_length
                + (i_length - ((i.abs() - i) / 2) % i_length) % i_length;
            let mut foreground_index = 0;
            let mut background = None;
            if self.selected == pythons_index {
                foreground_index = 1;
                background = Some(self.selected_background);
            }
            for (_, current_value) in self.options.iter().skip(pythons_index as usize).take(1) {
                let mut big_gap = gap;
                if pythons_index == self.selected {
                    //element get elevated, if they are currently selected
                    big_gap *= 2.0;
                }
                use super::text_render::text_render::draw_filled_string;
                draw_filled_string(
                    &current_value.0,
                    x + big_gap,
                    y,
                    self.block_width,
                    self.block_height,
                    current_value.2[foreground_index],
                    background,
                    transform,
                    graphics,
                );
                let line_color = current_value.2[foreground_index];
                let middle = y + current_value.1 * 0.5;
                piston_window::line(
                    line_color,
                    big_gap * 0.1,
                    [x, middle, x + big_gap * 0.8, middle],
                    transform,
                    graphics,
                );
                y += current_value.1 + gap;
            }
        }
        piston_window::line(line_color, gap * 0.1, [x, old_y, x, y], transform, graphics);
    }

    ///render the menu: render menu static: all options are always rendered, regardless of the view size
    pub fn render_static(
        &self,
        x: f64,
        mut y: f64,
        gap: f64,
        transform: [[f64; 3]; 2],
        graphics: &mut impl piston_window::Graphics,
        line_color: [f32; 4],
    ) {
        let i_length = self.options.len() as i64;
        let old_y = y;
        for (index, (_key, current_value)) in self.options.iter().enumerate() {
            let mut foreground_index = 0;
            let mut background = None;
            let mut big_gap = gap;
            if self.selected == index as i64{
                foreground_index = 1;
                background = Some(self.selected_background);
                big_gap *= 2.0;
            }
            use super::text_render::text_render::draw_filled_string;
            draw_filled_string(
                &current_value.0,
                x + big_gap,
                y,
                self.block_width,
                self.block_height,
                current_value.2[foreground_index],
                background,
                transform,
                graphics,
            );
            let line_color = current_value.2[foreground_index];
            let middle = y + current_value.1 * 0.5;
            piston_window::line(
                line_color,
                big_gap * 0.1,
                [x, middle, x + big_gap * 0.8, middle],
                transform,
                graphics,
            );
            y += current_value.1 + gap;
        }
        piston_window::line(line_color, gap * 0.1, [x, old_y, x, y], transform, graphics);
    }

    pub fn on_up(&mut self) -> Option<&T> {
        if self.selected == 0 {
            self.selected = self.options.len() as i64 - 1;
        } else {
            self.selected -= 1;
        }
        self.on_enter()
    }

    pub fn on_down(&mut self) -> Option<&T> {
        self.selected += 1;
        if self.selected >= self.options.len() as i64 {
            self.selected = 0;
        }
        self.on_enter()
    }

    pub fn on_enter(&mut self) -> Option<&T> {
        for selected in self.options.keys().skip(self.selected as usize).take(1) {
            return Some(selected);
        }
        None
    }

    pub fn add_text(
        &mut self,
        identifier: T,
        text: String,
        foreground: [f32; 4],
        background: [f32; 4],
    ) {
        use super::text_render::text_render::drawn_text_dimension;
        let dim = drawn_text_dimension(&text);
        self.options.insert(
            identifier,
            (
                text,
                dim[1] as f64 * 6.0 * self.block_height,
                [background, foreground],
            ),
        );
    }

    pub fn set_text(&mut self, identifier: &T, text: String) -> bool {
        if let Some(content) = self.options.get_mut(identifier) {
            content.0 = text;
            return true;
        }
        false
    }
}
