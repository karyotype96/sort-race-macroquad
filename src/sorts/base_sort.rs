use macroquad::{camera::{Camera2D, set_camera, set_default_camera}, color::{WHITE, hsl_to_rgb}, math::Rect, shapes::draw_rectangle, window::{screen_height, screen_width}};

#[derive(Clone, Copy, Debug)]
pub struct ReadData {
    pub index: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct SwapData {
    pub index1: usize,
    pub index2: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct WriteData {
    pub index: usize,
    pub from: u32,
    pub to: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum SortMove {
    None,
    Read(ReadData),
    Swap(SwapData),
    Write(WriteData),
}

pub enum SortType {
    Selection,
    Insertion,
    Bubble,
}

impl SortType {
    pub fn get_name(&self) -> &'static str {
        match *self {
            SortType::Selection => "Selection Sort",
            SortType::Insertion => "Insertion Sort",
            SortType::Bubble => "Bubble Sort"
        }
    }
}

pub trait Sort {
    // this will add the list of SortMoves for the sort struct
    fn init_sort(&mut self, data: &Vec<u32>);

    // during playback, this will execute the next SortMove
    fn advance_sort(&mut self);

    // move the sort back 1 tick
    fn withdraw_sort(&mut self);

    // returns true if the sort is finished
    fn is_finished(&self) -> bool;

    // should be different for each sort
    fn get_name(&self) -> &'static str;

    fn get_data(&self) -> Vec<u32>;
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
                SortMove::Read(data) => {
                    if i == data.index {
                        rectangle_color = WHITE;
                    }
                }
                SortMove::Swap(data) => {
                    if i == data.index1 || i == data.index2 {
                        rectangle_color = WHITE;
                    }
                }
                SortMove::Write(data) => {
                    if i == data.index {
                        rectangle_color = WHITE;
                    }
                }
            }

            let rectangle_height = gradient_value * screen_height();

            let x_offset = (i as f32) * rectangle_width;

            draw_rectangle(
                x_offset, 
                screen_height() - rectangle_height, 
                rectangle_width, rectangle_height, 
                rectangle_color
            );
        }

        set_default_camera();
    }
}