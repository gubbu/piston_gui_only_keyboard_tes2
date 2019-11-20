//every UI Element has to have a constant height, so that by whole number division the mouse_over
// element can be found by O(1)

pub struct VerticalMenu<T: std::cmp::Eq + std::hash::Hash> {
    pub selected: i64,
    pub view_up: i64,
    pub view_down: i64,
    pub block_width: f64,
    pub block_height: f64,
    //the text String and its height as f64, and its color information
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

    pub fn render(
        &self,
        x: f64,
        mut y: f64,
        gap: f64,
        transform: [[f64; 3]; 2],
        graphics: &mut impl piston_window::Graphics,
    ) {
        let i_length = self.options.len() as i64;
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
                use super::text_render::text_render::draw_filled_string;
                draw_filled_string(
                    &current_value.0,
                    x,
                    y,
                    self.block_width,
                    self.block_height,
                    current_value.2[foreground_index],
                    background,
                    transform,
                    graphics,
                );
                y += current_value.1 + gap;
            }
        }
        /*
        for (_key, value) in self.options.iter().skip(self.selected as usize).take(1) {
            use super::text_render::text_render::draw_filled_string;
            draw_filled_string(
                &value.0,
                x,
                y,
                self.block_width,
                self.block_height,
                value.2[1],
                None,
                transform,
                graphics,
            );
            y += value.1;
        }

        for (_key, value) in self
            .options
            .iter()
            .skip(self.selected as usize + 1)
            .take(self.view_size as usize - 1)
        {
            use super::text_render::text_render::draw_filled_string;
            draw_filled_string(
                &value.0,
                x,
                y,
                self.block_width,
                self.block_height,
                value.2[0],
                None,
                transform,
                graphics,
            );
            y += value.1;
        }
        */
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
