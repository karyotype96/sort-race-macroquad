use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct HeapSort {
    sort_info: SortBase
}

impl Sort for HeapSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            ..Default::default()
        };

        let mut pre_data = data.clone();
        let len = pre_data.len();

        for i in (0..pre_data.len()/2).rev() {
            self.heapify(&mut pre_data, len, i);
        }

        for i in (0..pre_data.len()).rev() {
            self.sort_info.swap(&mut pre_data, 0, i);

            self.heapify(&mut pre_data, i, 0);
        }
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String { String::from("Heap Sort") }
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

impl HeapSort {
    fn heapify(&mut self, data: &mut Vec<u32>, n: usize, i: usize) {
        let mut largest = i;

        let left = 2 * i + 1;
        let right = 2 * i + 2;

        self.sort_info.read(left);
        self.sort_info.read(largest);
        if left < n && data[left] > data[largest] {
            largest = left;
        }

        self.sort_info.read(right);
        self.sort_info.read(largest);
        if right < n && data[right] > data[largest] {
            largest = right;
        }

        if largest != i {
            self.sort_info.swap(data, i, largest);

            self.heapify(data, n, largest);
        }
    }
}