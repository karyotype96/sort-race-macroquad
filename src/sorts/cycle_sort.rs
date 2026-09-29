use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct CycleSort {
    sort_info: SortBase
}

impl Sort for CycleSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            move_index: 0,
        };

        let mut pre_data = data.clone();

        for cycle_start in 0..=pre_data.len()-2 {
            let mut item = pre_data[cycle_start];

            let mut pos = cycle_start;
            for i in cycle_start+1..pre_data.len() {
                if pre_data[i] < item {
                    pos += 1;
                    self.sort_info.moves.push(SortMove::Read { index: cycle_start });
                    self.sort_info.moves.push(SortMove::Read { index: pos });
                }
            }

            if pos == cycle_start {
                continue;
            }

            while item == pre_data[pos] {
                self.sort_info.moves.push(SortMove::Read { index: pos });
                pos += 1;
            }

            if pos != cycle_start {
                let tmp = item;
                self.sort_info.moves.push(SortMove::Read { index: pos });
                item = pre_data[pos];
                self.sort_info.moves.push(SortMove::Write { index: pos, from: pre_data[pos], to: tmp });
                pre_data[pos] = tmp;
            }

            while pos != cycle_start {
                pos = cycle_start;

                for i in cycle_start+1..pre_data.len() {
                    if pre_data[i] < item {
                        pos += 1;
                        self.sort_info.moves.push(SortMove::Read { index: cycle_start });
                        self.sort_info.moves.push(SortMove::Read { index: pos });
                    }
                }

                self.sort_info.moves.push(SortMove::Read { index: pos });
                while item == pre_data[pos] {
                    pos += 1;
                    self.sort_info.moves.push(SortMove::Read { index: pos });
                }

                if item != pre_data[pos] {
                    let tmp = item;
                    self.sort_info.moves.push(SortMove::Read { index: pos });
                    item = pre_data[pos];
                    self.sort_info.moves.push(SortMove::Write { index: pos, from: pre_data[pos], to: tmp });
                    pre_data[pos] = tmp;
                }

            }
        }
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> &'static str { "Cycle Sort" }
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