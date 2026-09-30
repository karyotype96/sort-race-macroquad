use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct RadixSortLSD {
    sort_info: SortBase,
    radix_base: u32,
}

impl Sort for RadixSortLSD {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            move_index: 0,
        };

        let mut pre_data = data.clone();
    
        let mut min_value = pre_data[0];
        let mut max_value = pre_data[0];

        for i in 1..pre_data.len() {
            if data[i] < min_value {
                min_value = data[i];
            } else if data[i] > max_value {
                max_value = data[i]
            }
        }

        let mut exponent = 1;
        while (max_value - min_value) / exponent >= 1 {
            self.count_sort_by_digit(&mut pre_data, exponent, min_value);
            exponent *= self.radix_base;
        }
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String { format!("Radix Sort (Base {})", self.radix_base) }
    fn get_move_index(&self) -> usize { self.sort_info.move_index }
    fn get_data(&self) -> Vec<u32> { self.sort_info.data.clone() }
    fn get_move_count(&self) -> usize { self.sort_info.moves.len() }

    fn get_current_move(&self) -> SortMove {
        if self.sort_info.is_finished() {
            SortMove::None
        } else {
            self.sort_info.moves[self.sort_info.move_index]
        }
    }
}

impl RadixSortLSD {
    pub fn new(radix_base: u32) -> Self {
        let sort_info = SortBase::default();

        Self {
            sort_info,
            radix_base
        }
    }

    fn count_sort_by_digit(&mut self, data: &mut Vec<u32>, exponent: u32, min_value: u32) {
        let radix = self.radix_base;

        let mut bucket_index;
        let mut buckets = vec![0; radix as usize];
        let mut outputs = vec![0; data.len()];

        for i in 0..data.len() {
            self.sort_info.moves.push(SortMove::Read { index: i });
            bucket_index = (((data[i] - min_value) / exponent) % radix) as usize;
            buckets[bucket_index] += 1;
        }

        for i in 1..(radix as usize) {
            buckets[i] += buckets[i-1];
        }

        for i in (0..data.len()).rev() {
            bucket_index = (((data[i] - min_value) / exponent) % radix) as usize;
            buckets[bucket_index] -= 1;
            outputs[buckets[bucket_index] as usize] = data[i];
        }

        for i in 0..data.len() {
            self.sort_info.moves.push(SortMove::Write { index: i, from: data[i], to: outputs[i] });
            data[i] = outputs[i];
        }
    }
}