use crate::sorts::base_sort::{Sort, SortBase, SortMove};

const LP: [usize; 22] = [
    1, 1, 3, 5, 9, 15, 25, 41, 67, 109,
    177, 287, 465, 753, 1219, 1973, 3193, 5167, 8361, 13529, 21891, 35421,
];

#[derive(Default)]
pub struct SmoothSort {
    sort_info: SortBase
}

impl Sort for SmoothSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            move_index: 0,
        };

        let mut pre_data = data.clone();

        self.smooth_sort(&mut pre_data, 0, data.len()-1);
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String { String::from("Smooth Sort") }
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

impl SmoothSort {
    fn smooth_sort(&mut self, data: &mut Vec<u32>, lo: usize, hi: usize){
        let mut head = lo;
        
        let mut p = 1;
        let mut pshift = 1;

        while head < hi {
            if (p & 3) == 3 {
                self.sift(data, pshift, head);
                p >>= 2;
                pshift += 2;
            } else {
                if LP[pshift - 1] >= hi - head {
                    self.trinkle(data, p, pshift, head, false);
                } else {
                    self.sift(data, pshift, head);
                }

                if pshift == 1 {
                    p <<= 1;
                    pshift -= 1;
                } else {
                    p <<= pshift - 1;
                    pshift = 1;
                }
            }

            p |= 1;
            head += 1;
        }

        self.trinkle(data, p, pshift, head, false);

        while pshift != 1 || p != 1 {
            if pshift <= 1 {
                let trail = (p & !1).trailing_zeros();
                p >>= trail;
                pshift += trail as usize;
            } else {
                p <<= 2;
                p ^= 7;
                pshift -= 2;

                self.trinkle(data, p >> 1, pshift + 1, head - LP[pshift] - 1, true);
                self.trinkle(data, p, pshift, head - 1, true);
            }

            head -= 1;
        }
    }

    fn sift(&mut self, data: &mut Vec<u32>, orig_pshift: usize, orig_head: usize) {
        let mut pshift = orig_pshift;
        let mut head = orig_head;
        self.sort_info.read(head);
        let val = data[head];

        while pshift > 1 {
            let rt = head - 1;
            let lf = head - 1 - LP[pshift - 2];

            self.sort_info.read(rt);
            self.sort_info.read(lf);

            if val > data[lf] && val > data[rt] {
                break;
            }

            if data[lf] > data[rt] {
                self.sort_info.write(data, head, data[lf]);
                head = lf;
                pshift -= 1;
            } else {
                self.sort_info.write(data, head, data[rt]);
                head = rt;
                pshift -= 2;
            }
        }

        self.sort_info.write(data, head, val);
    }

    fn trinkle(&mut self, data: &mut Vec<u32>, orig_p: usize, orig_pshift: usize, orig_head: usize, orig_is_trusty: bool) {
        let val = data[orig_head];
        let mut p = orig_p;
        let mut pshift = orig_pshift;
        let mut head = orig_head;
        let mut is_trusty = orig_is_trusty;

        while p != 1 {
            let stepson = head - LP[pshift];

            self.sort_info.read(stepson);
            if data[stepson] <= val {
                break;
            }

            if !is_trusty && pshift > 1 {
                let rt = head - 1;
                let lf = head - 1 - LP[pshift - 2];

                self.sort_info.read(rt);
                self.sort_info.read(stepson);
                self.sort_info.read(lf);
                self.sort_info.read(stepson);

                if data[rt] >= data[stepson] || data[lf] >= data[stepson] {
                    break;
                }
            }

            self.sort_info.read(stepson);
            self.sort_info.write(data, head, data[stepson]);

            head = stepson;
            let trail = (p & !1).trailing_zeros() as usize;
            p >>= trail;
            pshift += trail;
            is_trusty = false;
        }

        if !is_trusty {
            self.sort_info.write(data, head, val);
            self.sift(data, pshift, head);
        }
    }
}