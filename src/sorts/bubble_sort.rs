use crate::sorts::base_sort::{Sort, SortMove};

#[derive(Default)]
pub struct BubbleSort {
    data: Vec<u32>,
    selected: Option<usize>,
    moves: Vec<SortMove>,
    move_index: usize,
}

impl Sort for BubbleSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.data = data.clone();
        self.moves.clear();
        self.move_index = 0;

        let mut pre_data = data.clone();
        for i in 0..pre_data.len() {
            for j in 0..pre_data.len() - 1 {
                if pre_data[j] > pre_data[j+1] {
                    let tmp = pre_data[j];
                    pre_data[j] = pre_data[j+1];
                    pre_data[j+1] = tmp;
                    self.moves.push(SortMove::Swap(j, j+1));
                }
            }
        }
    }

    fn advance_sort(&mut self) {
        if self.is_finished() {
            return
        }

        match self.moves[self.move_index] {
            SortMove::Read(index) => {
                self.selected = Some(index);
            }
            SortMove::Swap(index1, index2) => {
                self.selected = None;
                let tmp = self.data[index1];
                self.data[index1] = self.data[index2];
                self.data[index2] = tmp;
            }
            SortMove::Write(index, value) => {
                self.selected = None;
                self.data[index] = value;
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.move_index >= self.moves.len()
    }
}