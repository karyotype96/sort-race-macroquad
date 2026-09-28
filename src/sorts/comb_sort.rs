use crate::sorts::base_sort::{Sort, SortMove};

#[derive(Default)]
pub struct CombSort {
    data: Vec<u32>,
    moves: Vec<SortMove>,
    move_index: usize,
}

impl Sort for CombSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.data = data.clone();
        self.moves.clear();
        self.move_index = 0;

        let mut pre_data = data.clone();
        let mut gap_length = pre_data.len() / 2;

        let mut is_sorted = false;
        while !is_sorted || gap_length > 1 {
            is_sorted = true;
            for i in 0..(pre_data.len()-gap_length){
                self.moves.push(SortMove::Read {
                    index: i,
                });
                self.moves.push(SortMove::Read {
                    index: i+gap_length,
                });

                if pre_data[i] > pre_data[i + gap_length] {
                    is_sorted = false;
                    let tmp = pre_data[i];
                    pre_data[i] = pre_data[i + gap_length];
                    pre_data[i + gap_length] = tmp;
                    self.moves.push(SortMove::Swap {
                        index1: i,
                        index2: i + gap_length
                    })
                }
            }

            gap_length = (gap_length as f64 / 1.3).floor() as usize;
            if gap_length < 1 {
                gap_length = 1;
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
        "Comb Sort"
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