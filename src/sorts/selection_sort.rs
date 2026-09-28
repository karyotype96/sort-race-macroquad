use crate::sorts::base_sort::{Sort, SortMove};

#[derive(Default)]
pub struct SelectionSort {
    pub data: Vec<u32>,
    pub moves: Vec<SortMove>,
    pub move_index: usize,
}

impl Sort for SelectionSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.data = data.clone();
        self.moves.clear();
        self.move_index = 0;

        let mut pre_data = data.clone();
        for i in 0..pre_data.len()-1 {
            let mut min_index = i;

            for j in i..pre_data.len() {
                self.moves.push(SortMove::Read { index: j });
                if pre_data[j] < pre_data[min_index] {
                    min_index = j;
                }
            }

            if i != min_index {
                let tmp = pre_data[i];
                pre_data[i] = pre_data[min_index];
                pre_data[min_index] = tmp;

                // self.moves.push(SortMove::Swap(i, min_index));
                self.moves.push(SortMove::Swap { index1: i, index2: min_index });
            }
        }
    }

    fn advance_sort(&mut self) {
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

    fn withdraw_sort(&mut self) {
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

    fn is_finished(&self) -> bool {
        self.move_index >= self.moves.len()
    }

    fn get_name(&self) -> &'static str {
        "Selection Sort"
    }

    fn get_data(&self) -> Vec<u32> {
        self.data.clone()
    }

    fn get_move_count(&self) -> usize {
        self.moves.len()
    }

    fn get_current_move(&self) -> SortMove {
        if self.is_finished() {
            SortMove::None
        } else {
            self.moves[self.move_index]
        }
    }
}