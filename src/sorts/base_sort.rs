use macroquad::{camera::{Camera2D, set_camera, set_default_camera}, color::{WHITE, YELLOW, hsl_to_rgb}, math::Rect, shapes::{draw_rectangle, draw_rectangle_lines}, text::draw_text, window::{screen_height, screen_width}};

use crate::helpers::map_range::map_range;

pub const FRAME_RATE: f32 = 60.0;

#[derive(Copy, Clone, Debug)]
pub enum SortMove {
    None,
    Read { index: usize },
    Swap { index1: usize, index2: usize },
    Write { index: usize, from: u32, to: u32 },
}

pub enum SortType {
    Selection,
    Insertion,
    Bubble,
    CocktailShaker,
    OutOfPlaceMerge,
    InPlaceMerge,
    QuickRP,
    Comb,
    Shell,
}

impl SortType {
    pub fn get_name(&self) -> &'static str {
        match *self {
            SortType::Selection => "Selection Sort",
            SortType::Insertion => "Insertion Sort",
            SortType::Bubble => "Bubble Sort",
            SortType::CocktailShaker => "Cocktail Shaker Sort",
            SortType::OutOfPlaceMerge => "Out-of-Place Merge Sort",
            SortType::InPlaceMerge => "In-Place Merge Sort",
            SortType::QuickRP => "Quick Sort - Right Pivot",
            SortType::Comb => "Comb Sort",
            SortType::Shell => "Shell Sort",
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
            SortMove::Swap { index1, index2 } => {
                let tmp = self.data[index1];
                self.data[index1] = self.data[index2];
                self.data[index2] = tmp;
            }
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
            SortMove::Swap { index1, index2 } => {
                let tmp = self.data[index1];
                self.data[index1] = self.data[index2];
                self.data[index2] = tmp;
            }
            SortMove::Write { index, from, .. } => {
                self.data[index] = from;
            }
            _ => {}
        }
    }

    pub fn is_finished(&self) -> bool {
        self.move_index >= self.moves.len()
    }
}

pub trait Sort {
    // this will add the list of SortMoves for the sort struct
    fn init_sort(&mut self, data: &Vec<u32>);

    // should be different for each sort
    fn get_name(&self) -> &'static str;

    fn advance_sort(&mut self);
    fn withdraw_sort(&mut self);

    fn get_data(&self) -> Vec<u32>;
    fn get_move_index(&self) -> usize;
    fn get_move_count(&self) -> usize;
    fn get_current_move(&self) -> SortMove;

    fn draw_sort(&self, viewport_rect: Rect) {
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
                SortMove::Swap { index1, index2 } => {
                    if i == index1 || i == index2 {
                        rectangle_color = WHITE;
                    }
                }
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

            draw_rectangle_lines(0.0, 0.0, screen_width(), screen_height(), 5.0, WHITE);
            draw_text(format!("{}", self.get_name()), 25.0, 50.0, 48.0, WHITE);
            if self.get_move_index() >= self.get_move_count(){
                draw_text("Finished!", 25.0, 100.0, 48.0, YELLOW);
            }
        }

        set_default_camera();
    }
}