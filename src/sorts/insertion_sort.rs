use crate::sorts::base_sort::{ReadData, Sort, SortMove, SwapData, WriteData};

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
        for i in 1..pre_data.len() {
            let mut j = i;
            self.moves.push(SortMove::Read(ReadData{ index: i }));
            while j > 0 && pre_data[j - 1] > pre_data[j] {
                let tmp = pre_data[j-1];
                pre_data[j-1] = pre_data[j];
                pre_data[j] = tmp;
                self.moves.push(SortMove::Swap(SwapData {
                    index1: j-1,
                    index2: j
                }));

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
            SortMove::Swap(mv) => {
                let tmp = self.data[mv.index1];
                self.data[mv.index1] = self.data[mv.index2];
                self.data[mv.index2] = tmp;
            }
            SortMove::Write(mv) => {
                self.data[mv.index] = mv.from;
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

    fn get_current_move(&self) -> SortMove {
        if self.is_finished() {
            SortMove::None
        } else {
            self.moves[self.move_index]
        }
    }
}