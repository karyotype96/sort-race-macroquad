use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct QuickSortRP {
    sort_info: SortBase
}

impl Sort for QuickSortRP {
    fn init_sort(&mut self, data: &Vec<u32>){
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            ..Default::default()
        };

        let mut pre_data = data.clone();
        let len: isize = pre_data.len() as isize;

        self.q_sort(&mut pre_data, 0, len-1);
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String { String::from("Quick Sort - Right Pivot") }
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

impl QuickSortRP {
    fn q_sort(&mut self, data: &mut Vec<u32>, low: isize, high: isize) {
        if low < high {
            let pi = self.partition(data, low, high);
            self.q_sort(data, low, pi - 1);
            self.q_sort(data, pi + 1, high);
        }
    }

    fn partition(&mut self, data: &mut Vec<u32>, low: isize, high: isize) -> isize {
        let pivot = data[high as usize];
        let mut i: isize = (low as isize) - 1;

        for j in low..high {
            self.sort_info.read(j as usize);
            if data[j as usize] < pivot {
                i += 1;
                self.sort_info.swap(data, i as usize, j as usize);
            }
        }

        self.sort_info.swap(data, (i+1) as usize, high as usize);

        i + 1
    }
}