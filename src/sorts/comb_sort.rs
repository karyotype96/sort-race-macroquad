use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct CombSort {
    sort_info: SortBase,
}

impl Sort for CombSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            move_index: 0,
        };

        let mut pre_data = data.clone();
        let mut gap_length = pre_data.len() / 2;

        let mut is_sorted = false;
        while !is_sorted || gap_length > 1 {
            is_sorted = true;
            for i in 0..(pre_data.len()-gap_length){
                self.sort_info.moves.push(SortMove::Read {
                    index: i,
                });
                self.sort_info.moves.push(SortMove::Read {
                    index: i+gap_length,
                });

                if pre_data[i] > pre_data[i + gap_length] {
                    is_sorted = false;
                    let tmp = pre_data[i];
                    pre_data[i] = pre_data[i + gap_length];
                    pre_data[i + gap_length] = tmp;
                    self.sort_info.moves.push(SortMove::Swap {
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

    fn get_name(&self) -> &'static str { "Comb Sort" }
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