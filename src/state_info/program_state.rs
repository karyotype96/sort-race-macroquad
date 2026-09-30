use macroquad::{input::{KeyCode, is_key_pressed}, rand::ChooseRandom};

use crate::{helpers::viewports::get_viewports, sorts::{
    base_sort::{Sort, SortType::{self}}, bubble_sort::BubbleSort, cocktail_shaker_sort::CocktailShakerSort, comb_sort::CombSort, cycle_sort::CycleSort, heap_sort::HeapSort, insertion_sort::*, merge_sort_ip::InPlaceMergeSort, merge_sort_oop::OutOfPlaceMergeSort, odd_even_sort::OddEvenSort, quick_sort::QuickSortRP, radix_sort::RadixSortLSD, selection_sort::*, shell_sort::ShellSort,
}};

pub enum ProgramState {
    SelectSorts,
    WatchSorts,
}

// Info for how each list starts
#[derive(Copy, Clone, Default)]
pub enum ListType {
    #[default]
    FullyRandom,
    SlightlyRandom,
    Reversed
}

#[derive(Default)]
pub struct WatchSortsState {
    starting_list: ListType,
    list_size: u32,
    sorts: Vec<Box<dyn Sort>>,
    is_playing: bool,
    play_speed: f64,
    frame: usize,
    sort_frame: usize,
    max_frame: usize,
    positions: Vec<PositionInfo>,
}

pub struct PositionInfo {
    orig_index: usize,
    move_count: usize,
    position: u8,
}

impl WatchSortsState {
    pub fn new(list_type: ListType, list_size: u32, sort_list: &Vec<&SortType>, play_speed: f64) -> Self {
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
                },
                SortType::CocktailShaker => {
                    let mut sort = CocktailShakerSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                },
                SortType::Cycle => {
                    let mut sort = CycleSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                }
                SortType::OddEven => {
                    let mut sort = OddEvenSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                }
                SortType::OutOfPlaceMerge => {
                    let mut sort = OutOfPlaceMergeSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                },
                SortType::InPlaceMerge => {
                    let mut sort = InPlaceMergeSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                },
                SortType::Heap => {
                    let mut sort = HeapSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                }
                SortType::QuickRP => {
                    let mut sort = QuickSortRP::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                },
                SortType::Comb => {
                    let mut sort = CombSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                },
                SortType::Shell => {
                    let mut sort = ShellSort::default();
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                },
                SortType::RadixLSD(radix) => {
                    let mut sort = RadixSortLSD::new(*radix);
                    sort.init_sort(&data);
                    sorts.push(Box::new(sort));
                }
            }
        }

        let mut i = 0;
        let mut position_infos: Vec<PositionInfo> = sorts.iter().map(|s| {
            i += 1;
            PositionInfo { orig_index: i-1, move_count: s.get_move_count(), position: 0 }
        }).collect();

        position_infos.sort_by(|l, r| {
            l.move_count.cmp(&r.move_count)
        });

        for i in 0..position_infos.len() {
            position_infos[i].position = i as u8;
        }

        position_infos.sort_by(|l, r| {
            l.orig_index.cmp(&r.orig_index)
        });

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
            sort_frame: 0,
            max_frame,
            positions: position_infos,
        }
    }

    pub fn play_or_pause(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn step_forward(&mut self) {
        if self.sort_frame < self.max_frame {
            for i in 0..self.sorts.len() {
                self.sorts[i].advance_sort();
            }
            self.sort_frame += 1;
        }
    }

    pub fn step_back(&mut self) {
        for i in 0..self.sorts.len() {
            self.sorts[i].withdraw_sort();
        }
        if self.sort_frame > 0 {
            self.sort_frame -= 1;
        }
    }

    pub fn reset_sorts(&mut self) {
        let data = randomize_data(self.starting_list, self.list_size);

        for i in 0..self.sorts.len() {
            self.sorts[i].init_sort(&data);
        }

        self.sort_frame = 0;
        self.frame = 0;

        let mut i = 0;
        let mut position_infos: Vec<PositionInfo> = self.sorts.iter().map(|s| {
            i += 1;
            PositionInfo { orig_index: i-1, move_count: s.get_move_count(), position: 0 }
        }).collect();

        position_infos.sort_by(|l, r| {
            l.move_count.cmp(&r.move_count)
        });

        for i in 0..position_infos.len() {
            position_infos[i].position = i as u8;
        }

        position_infos.sort_by(|l, r| {
            l.orig_index.cmp(&r.orig_index)
        });

        let max_frame = match self.sorts.iter().map(|sort| {
            sort.get_move_count()
        }).max() {
            Some(n) => n,
            None => panic!("Number of sorts should be 1 or greater")
        };

        self.max_frame = max_frame;
        self.positions = position_infos;
    }

    pub fn advance_frame(&mut self) {
        self.frame += 1;

        if self.play_speed > 1.0 {
            if self.frame >= self.play_speed as usize {
                self.step_forward();
                self.frame = 0;
            }
        } else {
            let mut count = 0.0;

            while count < 1.0 && self.sort_frame < self.max_frame {
                self.step_forward();
                count += self.play_speed;
            }
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

        if self.sorts.len() == 1{
            self.sorts[0].draw_sort(viewports[0], None);
        } else {
            for (i, sort) in self.sorts.iter().enumerate() {
                sort.draw_sort(viewports[i], Some(self.positions[i].position));
            }
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