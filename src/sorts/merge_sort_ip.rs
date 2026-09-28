use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct InPlaceMergeSort {
    pub sort_info: SortBase
}

impl Sort for InPlaceMergeSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            move_index: 0,
        };

        let mut pre_data = data.clone();
        let len = pre_data.len();

        self.merge(&mut pre_data, 0, len);
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> &'static str { "In-Place Merge Sort" }
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

impl InPlaceMergeSort {
    fn merge(&mut self, data: &mut Vec<u32>, low: usize, high: usize) {
        if high - low < 2 {
            return;
        }

        let halfway_point = (high - low) / 2 + low;

        self.merge(data, low, halfway_point);
        self.merge(data, halfway_point, high);

        for i in halfway_point..high {
            let mut j = i;
            self.sort_info.moves.push(SortMove::Read { index: j });
            while j > low && data[j] < data[j-1] {
                self.sort_info.moves.push(SortMove::Swap { index1: j-1, index2: j });
                let tmp = data[j-1];
                data[j-1] = data[j];
                data[j] = tmp;
                j -= 1;
            }
        }
    }
}