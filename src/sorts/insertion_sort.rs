use crate::sorts::base_sort::{Sort, SortMove};

#[derive(Default)]
pub struct InsertionSort {
    data: Vec<u32>,
    moves: Vec<SortMove>,
    move_index: usize,
}

impl Sort for InsertionSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.data = data.clone();
        self.moves.clear();
        self.move_index = 0;

        let mut pre_data = data.clone();
        for i in 1..pre_data.len() {
            let mut j = i;
            self.moves.push(SortMove::Read { index: i });
            while j > 0 && pre_data[j - 1] > pre_data[j] {
                let tmp = pre_data[j-1];
                pre_data[j-1] = pre_data[j];
                pre_data[j] = tmp;
                self.moves.push(SortMove::Swap {
                    index1: j-1,
                    index2: j
                });

                j -= 1;
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
        "Insertion Sort"
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