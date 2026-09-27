use crate::sorts::base_sort::{ReadData, Sort, SortMove, SwapData};

#[derive(Default)]
pub struct CocktailShakerSort {
    data: Vec<u32>,
    moves: Vec<SortMove>,
    move_index: usize,
}

impl Sort for CocktailShakerSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.data = data.clone();
        self.moves.clear();
        self.move_index = 0;

        let mut pre_data = data.clone();
        for _ in 0..pre_data.len() / 2 {
            let mut is_ordered = true;
            for j in 0..pre_data.len() - 1 {
                self.moves.push(SortMove::Read(ReadData {
                    index: j
                }));

                if pre_data[j] > pre_data[j+1] {
                    is_ordered = false;
                    let tmp = pre_data[j];
                    pre_data[j] = pre_data[j+1];
                    pre_data[j+1] = tmp;
                    self.moves.push(SortMove::Swap(SwapData {
                        index1: j,
                        index2: j+1,
                    }));
                }
            }
            if is_ordered {
                break;
            }

            for j in (0..pre_data.len() - 1).rev() {
                self.moves.push(SortMove::Read(ReadData {
                    index: j
                }));

                if pre_data[j] > pre_data[j+1] {
                    is_ordered = false;
                    let tmp = pre_data[j];
                    pre_data[j] = pre_data[j+1];
                    pre_data[j+1] = tmp;
                    self.moves.push(SortMove::Swap(SwapData {
                        index1: j,
                        index2: j+1,
                    }));
                }
            }
            if is_ordered {
                break;
            }
        }
    }

    fn advance_sort(&mut self) {
        if self.is_finished() {
            self.move_index += 1;
            return
        }

        match self.moves[self.move_index] {
            SortMove::Swap(mv) => {
                let tmp = self.data[mv.index1];
                self.data[mv.index1] = self.data[mv.index2];
                self.data[mv.index2] = tmp;
            }
            SortMove::Write(mv) => {
                self.data[mv.index] = mv.to;
            }
            _ => {}
        }

        self.move_index += 1;
    }

    fn withdraw_sort(&mut self) {
        if self.move_index == 0 {
            return;
        }

        self.move_index -= 1;
        if self.is_finished() {
            return;
        }

        match self.moves[self.move_index] {
            SortMove::Swap(mv) => {
                let tmp = self.data[mv.index1];
                self.data[mv.index1] = self.data[mv.index2];
                self.data[mv.index2] = tmp;
            }
            SortMove::Write(mv) => {
                self.data[mv.index] = mv.from;
            }
            _ => {}
        }
    }

    fn is_finished(&self) -> bool {
        self.move_index >= self.moves.len()
    }

    fn get_name(&self) -> &'static str {
        "Cocktail Shaker Sort"
    }

    fn get_data(&self) -> Vec<u32> {
        self.data.clone()
    }

    fn get_move_count(&self) -> usize {
        self.moves.len()
    }

    fn get_current_move(&self) -> SortMove {
        if self.is_finished() {
            SortMove::None
        } else {
            self.moves[self.move_index]
        }
    }
}