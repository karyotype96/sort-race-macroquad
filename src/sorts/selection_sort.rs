use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct SelectionSort {
    sort_info: SortBase
}

impl Sort for SelectionSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            move_index: 0,
        };

        let mut pre_data = data.clone();
        for i in 0..pre_data.len()-1 {
            let mut min_index = i;

            for j in i..pre_data.len() {
                self.sort_info.moves.push(SortMove::Read { index: j });
                if pre_data[j] < pre_data[min_index] {
                    min_index = j;
                }
            }

            if i != min_index {
                let tmp = pre_data[i];
                pre_data[i] = pre_data[min_index];
                pre_data[min_index] = tmp;

                self.sort_info.moves.push(SortMove::Swap { index1: i, index2: min_index });
            }
        }
    }

    fn advance_sort(&mut self) {
        if self.sort_info.is_finished() {
            self.sort_info.move_index += 1;
            return
        }

        match self.sort_info.moves[self.sort_info.move_index] {
            SortMove::Swap { index1, index2 } => {
                let tmp = self.sort_info.data[index1];
                self.sort_info.data[index1] = self.sort_info.data[index2];
                self.sort_info.data[index2] = tmp;
            }
            SortMove::Write { index, from: _, to } => {
                self.sort_info.data[index] = to;
            }
            _ => {}
        }

        self.sort_info.move_index += 1;
    }

    fn withdraw_sort(&mut self) {
        if self.sort_info.move_index == 0 {
            return;
        }

        self.sort_info.move_index -= 1;
        if self.sort_info.is_finished() {
            return;
        }

        match self.sort_info.moves[self.sort_info.move_index] {
            SortMove::Swap { index1, index2 } => {
                let tmp = self.sort_info.data[index1];
                self.sort_info.data[index1] = self.sort_info.data[index2];
                self.sort_info.data[index2] = tmp;
            }
            SortMove::Write { index, from, .. } => {
                self.sort_info.data[index] = from;
            }
            _ => {}
        }
    }

    fn get_name(&self) -> &'static str { "Selection Sort" }
    fn get_data(&self) -> Vec<u32> { self.sort_info.data.clone() }
    fn get_move_index(&self) -> usize { self.sort_info.move_index }
    fn get_move_count(&self) -> usize { self.sort_info.moves.len() }
    fn get_current_move(&self) -> SortMove {
        if self.sort_info.is_finished() {
            SortMove::None
        } else {
            self.sort_info.moves[self.sort_info.move_index]
        }
    }
}