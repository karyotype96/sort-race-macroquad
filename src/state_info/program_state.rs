use macroquad::{input::{KeyCode, is_key_pressed}, rand::ChooseRandom};

use crate::{helpers::viewports::get_viewports, sorts::{
    base_sort::{Sort, SortType}, bubble_sort::BubbleSort, insertion_sort::*, selection_sort::*,
}};

pub const FRAME_RATE: u32 = 60;
pub const SLOW_SPEED: u32 = FRAME_RATE / 6;
pub const MEDIUM_SPEED: u32 = FRAME_RATE / 12;
pub const HIGH_SPEED: u32 = FRAME_RATE / 30;

pub enum ProgramState {
    SelectListType,
    SelectSorts(ListType),
    WatchSorts,
}

// Info for how each list starts
#[derive(Copy, Clone)]
pub enum ListType {
    FullyRandom,
    SlightlyRandom,
    Reversed
}

pub struct WatchSortsState {
    starting_list: ListType,
    list_size: u32,
    sorts: Vec<Box<dyn Sort>>,
    is_playing: bool,
    play_speed: u32,
    frame: usize,
    max_frame: usize,
}

impl WatchSortsState {
    pub fn new(list_type: ListType, list_size: u32, sort_list: &Vec<SortType>, play_speed: u32) -> Self {
        let mut sorts: Vec<Box<dyn Sort>> = Vec::new();

        let data = randomize_data(list_type, list_size);

        for st in sort_list {
            match st {
                SortType::Selection => {
                    let mut sort = SelectionSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort))
                },
                SortType::Insertion => {
                    let mut sort = InsertionSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                },
                SortType::Bubble => {
                    let mut sort = BubbleSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                }
            }
        }

        let max_frame = match sorts.iter().map(|sort| {
            sort.get_move_count()
        }).max() {
            Some(n) => n,
            None => panic!("Number of sorts should be 1 or greater")
        };

        Self {
            starting_list: list_type,
            list_size,
            sorts,
            is_playing: false,
            play_speed,
            frame: 0,
            max_frame,
        }
    }

    pub fn play_or_pause(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn step_forward(&mut self) {
        for i in 0..self.sorts.len() {
            self.sorts[i].advance_sort();
        }
    }

    pub fn step_back(&mut self) {
        for i in 0..self.sorts.len() {
            self.sorts[i].withdraw_sort();
        }
    }

    pub fn reset_sorts(&mut self) {
        let data = randomize_data(self.starting_list, self.list_size);

        for i in 0..self.sorts.len() {
            self.sorts[i].init_sort(&data);
        }
    }

    pub fn advance_frame(&mut self) {
        self.frame += 1;
        if self.frame >= self.play_speed as usize {
            self.step_forward();
            self.frame = 0;
        }
    }

    pub fn control(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.play_or_pause();
            return;
        }

        if is_key_pressed(KeyCode::Right) && !self.is_playing {
            if self.frame <= self.max_frame {
                self.step_forward();
            }
        }
        if is_key_pressed(KeyCode::Left) && !self.is_playing {
            self.step_back();
        }

        if is_key_pressed(KeyCode::R) {
            self.is_playing = false;
            self.reset_sorts();
        }
        
        if self.is_playing {
            self.advance_frame();
        }
    }

    pub fn draw(&self) {
        let sort_count = self.sorts.len();
        let viewports = get_viewports(sort_count);

        println!("viewport count: {}", viewports.len());

        for (i, sort) in self.sorts.iter().enumerate() {
            sort.draw_sort(viewports[i]);
        }
    }
}

fn randomize_data(list_type: ListType, list_size: u32) -> Vec<u32> {
    let mut data: Vec<u32> = (1..=list_size).collect();

    match list_type {
        ListType::FullyRandom => {
            data.shuffle();
        },
        ListType::SlightlyRandom => {
            for _ in 0..(list_size/10) {
                let index1: usize = rand::random_range(0..list_size) as usize;
                let index2: usize = rand::random_range(0..list_size) as usize;

                let tmp = data[index1];
                data[index1] = data[index2];
                data[index2] = tmp;
            }
        },
        ListType::Reversed => {
            data.reverse();
        }
    };

    data
}