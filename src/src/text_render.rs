//19.11.2019: ported from rust sdl2 to piston (just changing the types the logic stayed)

mod text_signs {
    pub const LETTERS: [[u8; 5]; 26] = [
        //A
        [0b11111, 0b10001, 0b11111, 0b10001, 0b10001],
        //B
        [0b11110, 0b10001, 0b11110, 0b10001, 0b11110],
        //C
        [0b11111, 0b10000, 0b10000, 0b10000, 0b11111],
        //D
        [0b11110, 0b10001, 0b10001, 0b10001, 0b11110],
        //E
        [0b11111, 0b10000, 0b11110, 0b10000, 0b11111],
        //F
        [0b11111, 0b10000, 0b11110, 0b10000, 0b10000],
        //G
        [0b11111, 0b10000, 0b10011, 0b10001, 0b11111],
        //H
        [0b10001, 0b10001, 0b11111, 0b10001, 0b10001],
        //I
        [0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        //J
        [0b01111, 0b00001, 0b00001, 0b00001, 0b11111],
        //K
        [0b10001, 0b10010, 0b10100, 0b10010, 0b10001],
        //L
        [0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        //M
        [0b11111, 0b10101, 0b10101, 0b10101, 0b10001],
        //N
        [0b10001, 0b11001, 0b10101, 0b10011, 0b10001],
        //O
        [0b01110, 0b10001, 0b10001, 0b10001, 0b01110],
        //P
        [0b11111, 0b10001, 0b11111, 0b10000, 0b10000],
        //Q
        [0b11111, 0b10001, 0b10001, 0b11111, 0b00001],
        //R
        [0b11111, 0b10001, 0b11111, 0b10010, 0b10001],
        //S
        [0b11111, 0b10000, 0b11111, 0b00001, 0b11111],
        [0b11111, 0b00100, 0b00100, 0b00100, 0b00100],
        [0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        [0b10001, 0b10001, 0b01010, 0b01010, 0b00100],
        [0b10001, 0b10101, 0b10101, 0b10101, 0b11111],
        [0b10001, 0b01010, 0b00100, 0b01010, 0b10001],
        [0b10001, 0b10001, 0b11111, 0b00100, 0b00100],
        [0b11111, 0b00010, 0b00100, 0b01000, 0b11111],
    ];

    pub const NUMBERS: [[u8; 5]; 10] = [
        [0b11111, 0b10011, 0b10101, 0b11001, 0b11111],
        [0b01100, 0b10100, 0b00100, 0b00100, 0b00100],
        [0b11111, 0b00001, 0b01110, 0b10000, 0b11111],
        [0b11111, 0b00001, 0b01111, 0b00001, 0b11111],
        [0b10001, 0b10001, 0b11111, 0b00001, 0b00001],
        [0b11111, 0b10000, 0b11111, 0b00001, 0b11111],
        [0b11111, 0b10000, 0b11111, 0b10001, 0b11111],
        [0b11111, 0b00011, 0b00100, 0b01000, 0b10000],
        [0b11111, 0b10001, 0b11111, 0b10001, 0b11111],
        [0b11111, 0b10001, 0b11111, 0b00001, 0b11111],
    ];

    pub const SIGNS: [[u8; 5]; 13] = [
        //+
        [0b00100, 0b00100, 0b11111, 0b00100, 0b00100],
        //-
        [0b00000, 0b00000, 0b11111, 0b00000, 0b00000],
        //times *
        [0b00000, 0b01010, 0b00100, 0b01010, 0b00000],
        //division /
        [0b00100, 0b00000, 0b11111, 0b00000, 0b00100],
        //dot .
        [0b00000, 0b00000, 0b00000, 0b00000, 0b00100],
        //:
        [0b00100, 0b00000, 0b00000, 0b00000, 0b00100],
        // exclaim
        [0b00100, 0b00100, 0b00100, 0b00000, 0b00100],
        //?
        [0b11110, 0b10001, 0b00110, 0, 0b00100],
        //=
        [0, 0b11111, 0, 0b11111, 0],
        //_
        [0, 0, 0, 0, 0xff],
        //|
        [0b00100; 5],
        //[
        [0xff, 1 << 4, 1 << 4, 1 << 4, 0xff],
        //]
        [0xff, 1, 1, 1, 0xff],
    ];

    pub fn index_char(character: char) -> [u8; 5] {
        match character {
            'A' | 'a' => LETTERS[0],
            'B' | 'b' => LETTERS[1],
            'C' | 'c' => LETTERS[2],
            'D' | 'd' => LETTERS[3],
            'E' | 'e' => LETTERS[4],
            'F' | 'f' => LETTERS[5],
            'G' | 'g' => LETTERS[6],
            'H' | 'h' => LETTERS[7],
            'I' | 'i' => LETTERS[8],
            'J' | 'j' => LETTERS[9],
            'K' | 'k' => LETTERS[10],
            'L' | 'l' => LETTERS[11],
            'M' | 'm' => LETTERS[12],
            'N' | 'n' => LETTERS[13],
            'O' | 'o' => LETTERS[14],
            'P' | 'p' => LETTERS[15],
            'Q' | 'q' => LETTERS[16],
            'R' | 'r' => LETTERS[17],
            'S' | 's' => LETTERS[18],
            'T' | 't' => LETTERS[19],
            'U' | 'u' => LETTERS[20],
            'V' | 'v' => LETTERS[21],
            'W' | 'w' => LETTERS[22],
            'X' | 'x' => LETTERS[23],
            'Y' | 'y' => LETTERS[24],
            'Z' | 'z' => LETTERS[25],
            '0' => NUMBERS[0],
            '1' => NUMBERS[1],
            '2' => NUMBERS[2],
            '3' => NUMBERS[3],
            '4' => NUMBERS[4],
            '5' => NUMBERS[5],
            '6' => NUMBERS[6],
            '7' => NUMBERS[7],
            '8' => NUMBERS[8],
            '9' => NUMBERS[9],
            '+' => SIGNS[0],
            '-' => SIGNS[1],
            '*' => SIGNS[2],
            '/' => SIGNS[3],
            '.' => SIGNS[4],
            ':' => SIGNS[5],
            '!' => SIGNS[6],
            '?' => SIGNS[7],
            '=' => SIGNS[8],
            '_' => SIGNS[9],
            '|' => SIGNS[10],
            '[' | '(' | '{' => SIGNS[11],
            ']' | ')' | '}' => SIGNS[12],
            _ => NUMBERS[0],
        }
    }
}

pub mod text_render {
    use super::text_signs::*;
    extern crate piston_window;
    use piston_window::{rectangle, Graphics};
    ///the letter will have the following dimensions (in sdl2 pixels/ sdl2 rectangle unit(?))
    /// width: block_width * 5
    /// height: block_height * 5
    /// to render 2 letters next to each other correctly, there has to be a gap between them
    /// the gap has to be greater or equal to
    /// block_width to the right and block_height to the next line below.
    fn draw_filled_letter(
        character: &[u8; 5],
        x: f64,
        y: f64,
        block_width: f64,
        block_height: f64,
        foreground_color: [f32; 4],
        background_rgba: Option<[f32; 4]>,
        transform: [[f64; 3]; 2],
        graphics: &mut impl Graphics,
    ) {
        if let Some(background_rgba) = background_rgba {
            rectangle(
                background_rgba,
                [x, y, 6.0 * block_width, 6.0 * block_height],
                transform,
                graphics,
            );
        }
        for (index, row) in character.iter().enumerate() {
            for block in (0..5u8).rev() {
                if row & (1 << block) != 0 {
                    rectangle(
                        foreground_color,
                        [
                            (4 - block) as f64 * block_width + x,
                            y + index as f64 * block_height,
                            block_width,
                            block_height,
                        ],
                        transform,
                        graphics,
                    );
                }
            }
        }
    }

    pub fn draw_filled_string(
        text: &String,
        mut x: f64,
        mut y: f64,
        block_width: f64,
        block_height: f64,
        foreground_color: [f32; 4],
        background_rgba: Option<[f32; 4]>,
        transform: [[f64; 3]; 2],
        graphics: &mut impl Graphics,
    ) {
        let x_base_line = x;
        for character in text.chars() {
            if character == ' ' {
                x += 6.0 * block_width;
            } else if character == '\n' {
                y += 6.0 * block_height;
                x = x_base_line;
            } else {
                draw_filled_letter(
                    &index_char(character),
                    x,
                    y,
                    block_width,
                    block_height,
                    foreground_color,
                    background_rgba,
                    transform,
                    graphics,
                );
                x += 6.0 * block_width;
            }
        }
    }

    ///returns the dimensions of a text in characters [width, height].
    /// the block_width can be 0, if: a)the String is empty b)the String only consists of \n chars.
    pub fn drawn_text_dimension(text: &String) -> [u32; 2] {
        let mut measure_width = 0;
        let mut measure_current_line_width = 0;
        let mut measure_height = 1;
        for character in text.chars() {
            if character == '\n' {
                measure_height += 1;
                measure_width = measure_width.max(measure_current_line_width);
                measure_current_line_width = 0;
            } else {
                measure_current_line_width += 1;
            }
        }
        [
            measure_width.max(measure_current_line_width),
            measure_height,
        ]
    }
}

/*
mod beautified_text_render{
    pub enum ColoredText{
        //changing the foreground and background color:
        ColorChange([f32;4], Option<[f32;4]>),
        //the actual text information
        TextBlock(String)
    }

    /// if no color information is provided at the beginning: renders white: rgba: [1.0;4] and no background
    pub fn draw_colored_text(text: &Vec<ColoredText>, x: f64, y: f64, block_width: f64, block_height: f64){
        let current_foreground = [1.0;4];
        let current_background = None;
        for element in text.iter(){

        }
    }
}
*/
