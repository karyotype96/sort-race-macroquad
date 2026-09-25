use crate::sorts::base_sort::{ReadData, Sort, SortMove, WriteData};

#[derive(Default)]
pub struct InsertionSort {
    data: Vec<u32>,
    selected: Option<usize>,
    moves: Vec<SortMove>,
    move_index: usize,
}

impl Sort for InsertionSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.data = data.clone();
        self.moves.clear();
        self.move_index = 0;

        let mut pre_data = data.clone();
        for i in 1..pre_data.len()-1 {
            self.moves.push(SortMove::Read(ReadData { index: i }));
            let key = pre_data[i];

            for j in (0..i).rev() {
                if pre_data[j] > key {
                    let from = pre_data[j];
                    pre_data[j] = key;
                    self.moves.push(SortMove::Write(WriteData { index: j, from, to: key }));
                    break;
                }

                let from = pre_data[j+1];
                pre_data[j+1] = pre_data[j];
                self.moves.push(SortMove::Write(WriteData { 
                    index: j+1, 
                    from, 
                    to: pre_data[j+1]}));

                if j == 0 {
                    let from = pre_data[j];
                    pre_data[j] = key;
                    self.moves.push(SortMove::Write(WriteData {
                        index: j,
                        from,
                        to: key
                    }));
                }
            }

        }
    }

    fn advance_sort(&mut self) {
        if self.is_finished() {
            return
        }

        match self.moves[self.move_index] {
            SortMove::Read(mv) => {
                self.selected = Some(mv.index);
            }
            SortMove::Swap(mv) => {
                self.selected = None;
                let tmp = self.data[mv.index1];
                self.data[mv.index1] = self.data[mv.index2];
                self.data[mv.index2] = tmp;
            }
            SortMove::Write(mv) => {
                self.selected = None;
                self.data[mv.index] = mv.to;
            }
        }
    }

    fn withdraw_sort(&mut self) {
        if self.move_index == 0 {
            return;
        }

        self.move_index -= 1;
        match self.moves[self.move_index] {
            SortMove::Read(mv) => {
                self.selected = Some(mv.index);
            }
            SortMove::Swap(mv) => {
                self.selected = None;
                let tmp = self.data[mv.index1];
                self.data[mv.index1] = self.data[mv.index2];
                self.data[mv.index2] = tmp;
            }
            SortMove::Write(mv) => {
                self.selected = None;
                self.data[mv.index] = mv.from;
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.move_index >= self.moves.len()
    }
}