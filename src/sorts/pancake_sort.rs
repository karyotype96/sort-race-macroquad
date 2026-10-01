use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct PancakeSort {
    sort_info: SortBase
}

impl Sort for PancakeSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            ..Default::default()
        };

        let mut pre_data = data.clone();

        for curr_size in (2..=pre_data.len()).rev() {
            let mi = self.find_max(&mut pre_data, curr_size);
            if mi != curr_size - 1 {
                self.flip(&mut pre_data, mi);
                self.flip(&mut pre_data, curr_size-1);
            }
        }
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String{ String::from("Pancake Sort") }
    fn get_move_index(&self) -> usize { self.sort_info.move_index }
    fn get_data(&self) -> Vec<u32> { self.sort_info.data.clone() }
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

impl PancakeSort {
    fn flip(&mut self, data: &mut Vec<u32>, i_start: usize) {
        let mut i = i_start;
        let mut start = 0;

        while start < i {
            self.sort_info.swap(data, start, i);
            start += 1;
            i -= 1;
        }
    }

    fn find_max(&mut self, data: &Vec<u32>, n: usize) -> usize {
        let mut mi = 0;
        for i in 0..n {
            self.sort_info.read(i);
            self.sort_info.read(mi);
            if data[i] > data[mi] {
                mi = i;
            }
        }

        mi
    }
}