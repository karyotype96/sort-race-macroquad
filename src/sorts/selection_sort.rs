use crate::sorts::base_sort::{ReadData, Sort, SortMove, SwapData};

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
                self.moves.push(SortMove::Read(ReadData { index: j }));
                if pre_data[j] < pre_data[min_index] {
                    min_index = j;
                }
            }

            if i != min_index {
                let tmp = pre_data[i];
                pre_data[i] = pre_data[min_index];
                pre_data[min_index] = tmp;

                // self.moves.push(SortMove::Swap(i, min_index));
                self.moves.push(SortMove::Swap(SwapData { index1: i, index2: min_index }));
            }
        }
    }

    fn advance_sort(&mut self) {
        if self.is_finished() {
            return
        }

        match self.moves[self.move_index] {
            SortMove::Read(mv) => {
                self.selected = Some(mv.index);
            }
            SortMove::Swap(mv) => {
                self.selected = None;
                let tmp = self.data[mv.index1];
                self.data[mv.index1] = self.data[mv.index2];
                self.data[mv.index2] = tmp;
            }
            SortMove::Write(mv) => {
                self.selected = None;
                self.data[mv.index] = mv.to;
            }
        }
    }

    fn withdraw_sort(&mut self) {
        if self.move_index == 0 {
            return;
        }

        self.move_index -= 1;
        match self.moves[self.move_index] {
            SortMove::Read(mv) => {
                self.selected = Some(mv.index);
            }
            SortMove::Swap(mv) => {
                self.selected = None;
                let tmp = self.data[mv.index1];
                self.data[mv.index1] = self.data[mv.index2];
                self.data[mv.index2] = tmp;
            }
            SortMove::Write(mv) => {
                self.selected = None;
                self.data[mv.index] = mv.from;
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.move_index >= self.moves.len()
    }

    fn get_name(&self) -> &'static str {
        "Selection Sort"
    }
}