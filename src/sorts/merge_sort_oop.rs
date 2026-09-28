use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct OutOfPlaceMergeSort {
    pub sort_info: SortBase
}

impl Sort for OutOfPlaceMergeSort {
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

    fn get_name(&self) -> &'static str { "Out-of-Place Merge Sort" }
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

impl OutOfPlaceMergeSort {
    fn merge(&mut self, data: &mut Vec<u32>, low: usize, high: usize) {
        if high - low < 2 {
            return;
        }

        let halfway_point = (high - low) / 2 + low;

        self.merge(data, low, halfway_point);
        self.merge(data, halfway_point, high);

        let mut left: Vec<u32> = Vec::with_capacity(halfway_point - low);
        let mut right: Vec<u32> = Vec::with_capacity(high - halfway_point);

        for i in low..high {
            self.sort_info.moves.push(SortMove::Read { index: i });
            if i < halfway_point {
                left.push(data[i]);
            } else {
                right.push(data[i]);
            }
        }

        let mut left_index = 0;
        let mut right_index = 0;

        let mut arr_index = low;

        while left_index < left.len() && right_index < right.len() && arr_index < high {
            if left[left_index] <= right[right_index] {
                self.sort_info.moves.push(SortMove::Write { index: arr_index, from: data[arr_index], to: left[left_index] });
                data[arr_index] = left[left_index];
                left_index += 1;
            } else {
                self.sort_info.moves.push(SortMove::Write { index: arr_index, from: data[arr_index], to: right[right_index] });
                data[arr_index] = right[right_index];
                right_index += 1;
            }
            arr_index += 1;
        }

        while left_index < left.len() {
            self.sort_info.moves.push(SortMove::Write { index: arr_index, from: data[arr_index], to: left[left_index] });
            data[arr_index] = left[left_index];
            arr_index += 1;
            left_index += 1;
        }

        while right_index < right.len() {
            self.sort_info.moves.push(SortMove::Write { index: arr_index, from: data[arr_index], to: right[right_index] });
            data[arr_index] = right[right_index];
            arr_index += 1;
            right_index += 1;
        }
    }
}