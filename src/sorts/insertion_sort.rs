use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct InsertionSort {
    sort_info: SortBase,
}

impl Sort for InsertionSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            move_index: 0,
        };

        let mut pre_data = data.clone();
        for i in 1..pre_data.len() {
            let mut j = i;
            self.sort_info.moves.push(SortMove::Read { index: i });
            while j > 0 && pre_data[j - 1] > pre_data[j] {
                let tmp = pre_data[j-1];
                pre_data[j-1] = pre_data[j];
                pre_data[j] = tmp;
                self.sort_info.moves.push(SortMove::Swap {
                    index1: j-1,
                    index2: j
                });

                j -= 1;
            }
        }
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String { String::from("Insertion Sort") }
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