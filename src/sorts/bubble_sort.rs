use crate::sorts::base_sort::{Sort, SortMove, SwapData};

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
        for _ in 0..pre_data.len() {
            for j in 0..pre_data.len() - 1 {
                if pre_data[j] > pre_data[j+1] {
                    let tmp = pre_data[j];
                    pre_data[j] = pre_data[j+1];
                    pre_data[j+1] = tmp;
                    self.moves.push(SortMove::Swap(SwapData {
                        index1: j,
                        index2: j+1,
                    }));
                }
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
        "Bubble Sort"
    }
}