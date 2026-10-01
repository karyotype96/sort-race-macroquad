use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct GravitySort {
    sort_info: SortBase,
}

impl Sort for GravitySort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            ..Default::default()
        };

        let mut pre_data = data.clone();
        let len = pre_data.len();

        let mut min = pre_data[0];
        let mut max = pre_data[0];

        for i in 1..len {
            self.sort_info.read(i);
            if pre_data[i] < min {
                min = pre_data[i];
            }
            self.sort_info.read(i);
            if pre_data[i] > max {
                max = pre_data[i];
            }
        }

        let mut x = vec![0; len];
        let mut y = vec![0; (max-min+1) as usize];

        for i in 0..len {
            self.sort_info.do_nothing();
            x[i] = pre_data[i]-min;
            self.sort_info.do_nothing();
            y[pre_data[i] as usize - min as usize] = y[pre_data[i] as usize - min as usize]+1;
        }

        for i in (1..y.len()).rev() {
            self.sort_info.do_nothing();
            self.sort_info.do_nothing();
            y[i-1] += y[i];
            // self.sort_info.do_nothing();
            // self.sort_info.do_nothing();
        }

        for j in (0..y.len()).rev() {
            self.sort_info.read(len-y[j]);

            for i in 0..len {
                self.sort_info.read(i);

                let inc = if i >= len-y[j] { 1 } else { 0 };
                let dec = if x[i] >= j as u32 { 1 } else { 0 };
                let val = (pre_data[i]+inc).saturating_sub(dec);

                self.sort_info.write(&mut pre_data, i, val);
            }
        }
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String{ String::from("Gravity Sort") }
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