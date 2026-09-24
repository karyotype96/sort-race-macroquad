use crate::sorts::base_sort::{Sort, SortMove};

// Selection Sort
#[derive(Default)]
pub struct SelectionSort {
    pub data: Vec<u32>,
    pub selected: Option<usize>,
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
                self.moves.push(SortMove::Read(j));
                if pre_data[j] < pre_data[min_index] {
                    min_index = j;
                }
            }

            if i != min_index {
                let tmp = pre_data[i];
                pre_data[i] = pre_data[min_index];
                pre_data[min_index] = tmp;

                self.moves.push(SortMove::Swap(i, min_index));
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

        self.move_index += 1;
    }

    fn is_finished(&self) -> bool {
        self.move_index >= self.moves.len()
    }
}