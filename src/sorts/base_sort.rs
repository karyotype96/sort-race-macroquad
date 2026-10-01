use macroquad::{camera::{Camera2D, set_camera, set_default_camera}, color::{WHITE, YELLOW, hsl_to_rgb}, math::Rect, shapes::{draw_rectangle, draw_rectangle_lines}, text::draw_text, window::{screen_height, screen_width}};

use crate::helpers::map_range::map_range;

#[derive(Copy, Clone, Debug)]
pub enum SortMove {
    None,
    Read { index: usize },
    // Swap { index1: usize, index2: usize },
    Write { index: usize, from: u32, to: u32 },
}

pub enum SortType {
    Selection,
    Insertion,
    Bubble,
    CocktailShaker,
    Cycle,
    OddEven,
    OutOfPlaceMerge,
    InPlaceMerge,
    Heap,
    Smooth,
    QuickRP,
    Comb,
    Shell,
    RadixLSD(u32),
}

impl SortType {
    pub fn get_name(&self) -> String {
        match *self {
            SortType::Selection => String::from("Selection Sort"),
            SortType::Insertion => String::from("Insertion Sort"),
            SortType::Bubble => String::from("Bubble Sort"),
            SortType::CocktailShaker => String::from("Cocktail Shaker Sort"),
            SortType::Cycle => String::from("Cycle Sort"),
            SortType::OddEven => String::from("Odd-Even Sort"),
            SortType::OutOfPlaceMerge => String::from("Out-of-Place Merge Sort"),
            SortType::InPlaceMerge => String::from("In-Place Merge Sort"),
            SortType::Heap => String::from("Heap Sort"),
            SortType::Smooth => String::from("Smooth Sort"),
            SortType::QuickRP => String::from("Quick Sort - Right Pivot"),
            SortType::Comb => String::from("Comb Sort"),
            SortType::Shell => String::from("Shell Sort"),
            SortType::RadixLSD(radix) => format!("Radix Sort (Base {})", radix),
        }
    }
}

#[derive(Default)]
pub struct SortBase {
    pub data: Vec<u32>,
    pub moves: Vec<SortMove>,
    pub move_index: usize,
}

impl SortBase {
    // during playback, this will execute the next SortMove
    pub fn advance_sort(&mut self) {
        if self.is_finished() {
            self.move_index += 1;
            return
        }

        match self.moves[self.move_index] {
            /* SortMove::Swap { index1, index2 } => {
                let tmp = self.data[index1];
                self.data[index1] = self.data[index2];
                self.data[index2] = tmp;
            } */
            SortMove::Write { index, from: _, to } => {
                self.data[index] = to;
            }
            _ => {}
        }

        self.move_index += 1;
    }

    // move the sort back 1 tick
    pub fn withdraw_sort(&mut self) {
        if self.move_index == 0 {
            return;
        }

        self.move_index -= 1;
        if self.is_finished() {
            return;
        }

        match self.moves[self.move_index] {
            /* SortMove::Swap { index1, index2 } => {
                let tmp = self.data[index1];
                self.data[index1] = self.data[index2];
                self.data[index2] = tmp;
            } */
            SortMove::Write { index, from, .. } => {
                self.data[index] = from;
            }
            _ => {}
        }
    }

    pub fn is_finished(&self) -> bool {
        self.move_index >= self.moves.len()
    }

    pub fn read(&mut self, index: usize) {
        self.moves.push(SortMove::Read { index });
    }

    pub fn swap(&mut self, data: &mut Vec<u32>, index1: usize, index2: usize) {
        // self.moves.push(SortMove::Swap{ index1, index2 });
        self.moves.push(SortMove::Read { index: index1 });
        let tmp = data[index1];
        self.moves.push(SortMove::Write { index: index1, from: data[index1], to: data[index2] });
        data[index1] = data[index2];
        self.moves.push(SortMove::Write { index: index2, from: data[index2], to: tmp });
        data[index2] = tmp;
    }

    pub fn write(&mut self, data: &mut Vec<u32>, index: usize, val: u32) {
        self.moves.push(SortMove::Write { index, from: data[index], to: val });
        data[index] = val;
    }
}

pub trait Sort {
    // this will add the list of SortMoves for the sort struct
    fn init_sort(&mut self, data: &Vec<u32>);

    // should be different for each sort
    fn get_name(&self) -> String;

    fn advance_sort(&mut self);
    fn withdraw_sort(&mut self);

    fn get_data(&self) -> Vec<u32>;
    fn get_move_index(&self) -> usize;
    fn get_move_count(&self) -> usize;
    fn get_current_move(&self) -> SortMove;

    fn draw_sort(&self, viewport_rect: Rect, position: Option<u8>) {
        let mut camera1 = Camera2D::from_display_rect(
            Rect {
                x: 0.0, 
                y: screen_height(), 
                w: screen_width(),
                h: -screen_height()
            }
        );
        camera1.viewport = Some((
            viewport_rect.x as i32,
            viewport_rect.y as i32,
            viewport_rect.w as i32,
            viewport_rect.h as i32
        ));

        set_camera(&camera1);
        let data = self.get_data();

        let rectangle_width = screen_width() / (data.len() as f32);

        for (i, &v) in data.iter().enumerate() {
            let gradient_value = (v as f32) / data.len() as f32;
            let mut rectangle_color = hsl_to_rgb(gradient_value, 1.0, 0.5);

            match self.get_current_move() {
                SortMove::None => {}
                SortMove::Read { index } => {
                    if i == index {
                        rectangle_color = WHITE;
                    }
                }
                /* SortMove::Swap { index1, index2 } => {
                    if i == index1 || i == index2 {
                        rectangle_color = WHITE;
                    }
                } */
                SortMove::Write { index, .. } => {
                    if i == index {
                        rectangle_color = WHITE;
                    }
                }
            }

            let rectangle_height = map_range(gradient_value, 0.0, 1.0, 0.0, screen_height() - 50.0);

            let x_offset = (i as f32) * rectangle_width;

            draw_rectangle(
                x_offset, 
                screen_height() - rectangle_height, 
                rectangle_width, rectangle_height, 
                rectangle_color
            );

            draw_rectangle_lines(0.0, 0.0, screen_width(), screen_height(), 10.0, WHITE);
            draw_text(format!("{}", self.get_name()), 25.0, 50.0, 48.0, WHITE);

            if self.get_move_index() >= self.get_move_count(){
                match position {
                    Some(pos) => {
                        let pos_suffix = match pos+1 {
                            1 => "st",
                            2 => "nd",
                            3 => "rd",
                            _ => "th"
                        };
                        draw_text(format!("Finished ({}{} place)!", pos+1, pos_suffix), 25.0, 100.0, 48.0, YELLOW);
                    }
                    None => { draw_text("Finished!", 25.0, 100.0, 48.0, YELLOW); }
                }
            }
        }

        set_default_camera();
    }
}