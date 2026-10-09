use crate::sorts::base_sort::{Sort, SortMove};

// NOTE: Hanoi Sort does not at all work the same way as other
// sorts, so the framework for this one will have to be built
// from the ground up.
#[derive(Clone, Debug, Default)]
pub struct HanoiSortBase {
    pub data: Vec<Vec<u32>>,
    pub moves: Vec<HanoiSortMove>,
    pub move_index: usize,
    pub reads: usize,
    pub writes: usize,
}

#[derive(Clone, Debug, Default)]
pub struct HanoiSort {
    sort_info: HanoiSortBase
}

#[derive(Copy, Clone, Debug)]
pub enum HanoiSortMove {
    None,
    Read { tower: usize, index: usize },
    Move { from: usize, to: usize },
}

impl HanoiSortBase {
    pub fn advance_sort(&mut self) {
        if self.is_finished() {
            self.move_index += 1;
            return
        }

        match self.moves[self.move_index] {
            HanoiSortMove::Read { tower: _, index: _ } => { self.reads += 1 }
            HanoiSortMove::Move { from, to } => {
                match self.data[from].pop() {
                    Some(d) => {
                        self.data[to].push(d);
                        self.reads += 1;
                        self.writes += 1;
                    }
                    None => {}
                }
            }
            _ => {}
        }
        self.move_index += 1;
    }

    pub fn withdraw_sort(&mut self) {
        if self.move_index == 0 {
            return;
        }

        self.move_index -= 1;
        if self.is_finished() {
            return;
        }

        match self.moves[self.move_index] {
            HanoiSortMove::Read { tower: _, index: _ } => { self.reads -= 1 }
            HanoiSortMove::Move { from, to } => {
                if let Some(val) = self.data[to].pop() {
                    self.data[from].push(val);
                    self.reads -= 1;
                    self.writes -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn is_finished(&self) -> bool {
        self.move_index >= self.moves.len()
    }

    pub fn read(&mut self, tower: usize, index: usize) {
        self.moves.push(HanoiSortMove::Read { tower, index });
    }

    pub fn move_element(&mut self, data: &mut Vec<Vec<u32>>, from: usize, to: usize) {
        self.moves.push(HanoiSortMove::Move { from, to });
        match data[from].pop() {
            Some(d) => {
                data[to].push(d);
            }
            None => {}
        }
    }

    pub fn do_nothing(&mut self) {
        self.moves.push(HanoiSortMove::None);
    }
}

impl Sort for HanoiSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = HanoiSortBase { 
            data: vec![data.clone(), Vec::new(), Vec::new()],
            moves: Vec::new(),
            ..Default::default()
        };

        self.sort_info.data[0].reverse();

        let mut pre_data = vec![self.sort_info.data[0].clone(), Vec::new(), Vec::new()];

        let mut depth_index = 0;
        while let (Some(val1), Some(val2)) = (self.peek_at_depth(&pre_data, 0, depth_index), self.peek_at_depth(&pre_data, 0, depth_index + 1)) {
            if val1 > val2 {
                break;
            }

            depth_index += 1;
        }

        self.move_stack_to(&mut pre_data, 0, 2, depth_index + 1);

        while let Some(to_move) = self.peek_at_depth(&pre_data, 0, 0) {
            let mut tower_2_depth = 0;

            while let Some(val) = self.peek_at_depth(&pre_data, 2, tower_2_depth) {
                if val > to_move {
                    break;
                }

                tower_2_depth += 1;
            }

            self.move_stack_to(&mut pre_data, 2, 1, tower_2_depth);
            self.move_stack_to(&mut pre_data, 0, 2, 1);
            self.move_stack_to(&mut pre_data, 1, 2, tower_2_depth);
        }
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String { format!("Hanoi Sort") }
    fn get_move_index(&self) -> usize { self.sort_info.move_index }
    fn get_data(&self) -> Vec<u32> {
        let mut result = self.sort_info.data[0].clone();
        result.reverse();

        let mut tower2 = self.sort_info.data[1].clone();
        tower2.reverse();
        let mut tower3 = self.sort_info.data[2].clone();
        tower3.reverse();

        result.append(&mut tower2);
        result.append(&mut tower3);

        result
    }

    fn get_move_count(&self) -> usize { self.sort_info.moves.len() }
    fn get_reads(&self) -> usize { self.sort_info.reads }
    fn get_writes(&self) -> usize { self.sort_info.writes }

    fn get_current_move(&self) -> SortMove {
        if self.sort_info.is_finished() {
            SortMove::None
        } else {
            let s_move: SortMove = match self.sort_info.moves[self.sort_info.move_index] {
                HanoiSortMove::Read { tower, index } => {
                    let index = match tower {
                        0 => index,
                        1 => self.sort_info.data[0].len() + index,
                        _ => self.sort_info.data[0].len() + self.sort_info.data[1].len() + index,
                    };

                    SortMove::Read { index }
                }
                HanoiSortMove::Move { from: _, to } => {
                    let index = match to {
                        0 => 0,
                        1 => self.sort_info.data[0].len(),
                        _ => self.sort_info.data[0].len() + self.sort_info.data[1].len()
                    };

                    SortMove::Read { index }
                }
                _ => SortMove::None
            };

            s_move
        }
    }
}

impl HanoiSort {
    fn peek_at_depth(&mut self, towers: &Vec<Vec<u32>>, tower_index: usize, depth: usize) -> Option<u32> {
        if depth >= towers[tower_index].len() {
            return None;
        }

        let index = towers[tower_index].len() - depth - 1;

        self.sort_info.read(tower_index, index);
        return Some(towers[tower_index][index]);
    }

    fn move_stack_to(&mut self, towers: &mut Vec<Vec<u32>>, from: usize, to: usize, depth: usize) {
        if depth < 1 {
            return;
        }

        if depth == 1 {
            self.sort_info.move_element(towers, from, to);
            /*
            match towers[from].pop() {
                Some(data) => {
                    self.sort_info.move_element(towers, from, to);
                    towers[to].push(data);
                }
                None => {}
            }
            */
            return;
        }

        let next_to = match (from, to) {
            (0, 1) => 2,
            (0, 2) => 1,
            (1, 0) => 2,
            (1, 2) => 0,
            (2, 0) => 1,
            (2, 1) => 0,
            (_, _) => 0,
        };

        self.move_stack_to(towers, from, next_to, depth-1);
        self.sort_info.move_element(towers, from, to);
        /* match towers[from].pop() {
            Some(data) => {
                self.sort_info.move_element(towers, from, to);
                towers[to].push(data);
            }
            None => {}
        } */
        self.move_stack_to(towers, next_to, to, depth-1);
    }
}