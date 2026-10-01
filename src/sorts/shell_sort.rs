use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct ShellSort {
    sort_info: SortBase
}

impl Sort for ShellSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            move_index: 0,
        };

        let mut pre_data = data.clone();
        let mut gap = data.len() as isize;

        while gap > 1 {
            gap /= 2;

            for i in gap..(pre_data.len() as isize) {
                let mut j: isize = i as isize;
                self.sort_info.read(j as usize);

                while j - gap >= 0 && pre_data[j as usize] < pre_data[(j - gap) as usize] {
                    self.sort_info.swap(&mut pre_data, j as usize, (j - gap) as usize);

                    j -= gap;
                }
            }
        }
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String { String::from("Shell Sort") }
    fn get_move_index(&self) -> usize { self.sort_info.move_index }
    fn get_data(&self) -> Vec<u32> { self.sort_info.data.clone() }
    fn get_move_count(&self) -> usize { self.sort_info.moves.len() }

    fn get_current_move(&self) -> SortMove {
        if self.sort_info.is_finished() {
            SortMove::None
        } else {
            self.sort_info.moves[self.sort_info.move_index]
        }
    }
}