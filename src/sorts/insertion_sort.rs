use crate::sorts::base_sort::{Sort, SortMove};

#[derive(Default)]
pub struct InsertionSort {
    data: Vec<u32>,
    selected: Option<usize>,
    moves: Vec<SortMove>,
    move_index: usize,
}

impl Sort for InsertionSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.data = data.clone();
        self.moves.clear();
        self.move_index = 0;

        let mut pre_data = data.clone();
        for i in 1..pre_data.len()-1 {
            self.moves.push(SortMove::Read(i));
            let key = pre_data[i];

            for j in (0..i).rev() {
                if pre_data[j] > key {
                    pre_data[j] = key;
                    self.moves.push(SortMove::Write(j, key));
                    break;
                }

                pre_data[j+1] = pre_data[j];
                self.moves.push(SortMove::Write(j+1, pre_data[j+1]));

                if j == 0 {
                    pre_data[j] = key;
                    self.moves.push(SortMove::Write(j, key));
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