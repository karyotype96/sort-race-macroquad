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
            ..Default::default()
        };

        let mut pre_data = data.clone();
        let len = pre_data.len();
        
        self.merge(&mut pre_data, 0, len);
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String { String::from("Out-of-Place Merge Sort") }
    fn get_data(&self) -> Vec<u32> { self.sort_info.data.clone() }
    fn get_move_index(&self) -> usize { self.sort_info.move_index }
    fn get_move_count(&self) -> usize { self.sort_info.moves.len() }
    fn get_reads(&self) -> usize { self.sort_info.reads }
    fn get_writes(&self) -> usize { self.sort_info.writes }
    
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
            self.sort_info.read(i);
            self.sort_info.do_nothing();
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
            self.sort_info.do_nothing();
            self.sort_info.do_nothing();
            if left[left_index] <= right[right_index] {
                self.sort_info.write(data, arr_index, left[left_index]);
                left_index += 1;
            } else {
                self.sort_info.write(data, arr_index, right[right_index]);
                right_index += 1;
            }
            arr_index += 1;
        }

        while left_index < left.len() {
            self.sort_info.write(data, arr_index, left[left_index]);
            arr_index += 1;
            left_index += 1;
        }

        while right_index < right.len() {
            self.sort_info.write(data, arr_index, right[right_index]);
            arr_index += 1;
            right_index += 1;
        }
    }
}