use crate::sorts::base_sort::{Sort, SortBase, SortMove};

#[derive(Default)]
pub struct AmericanFlagSort {
    sort_info: SortBase,
    bucket_count: u32,
}

impl Sort for AmericanFlagSort {
    fn init_sort(&mut self, data: &Vec<u32>) {
        self.sort_info = SortBase {
            data: data.clone(),
            moves: Vec::new(),
            ..Default::default()
        };

        let mut pre_data = data.clone();
        let len = pre_data.len();

        let number_of_digits = self.get_max_number_of_digits(&mut pre_data, len);
        let mut max = 1;

        for _ in 0..number_of_digits-1 {
            max *= self.bucket_count;
        }

        self.sort(&mut pre_data, 0, len, max);
    }

    fn advance_sort(&mut self) { self.sort_info.advance_sort(); }
    fn withdraw_sort(&mut self) { self.sort_info.withdraw_sort(); }

    fn get_name(&self) -> String { format!("American Flag Sort ({} buckets)", self.bucket_count) }
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

impl AmericanFlagSort {
    pub fn new(bucket_count: u32) -> Self {
        let sort_info = SortBase::default();

        Self {
            sort_info,
            bucket_count
        }
    }

    fn get_max_number_of_digits(&mut self, data: &mut Vec<u32>, len: usize) -> u32 {
        let mut max = 0;
        let mut tmp;

        for i in 0..len {
            self.sort_info.read(i);
            tmp = ((data[i] as f32).log(self.bucket_count as f32) as u32) + 1;

            if tmp > max {
                max = tmp;
            }
        }

        max
    }

    fn get_digit(&self, integer: u32, divisor: u32) -> u32 {
        (integer / divisor) % self.bucket_count
    }

    fn sort(&mut self, data: &mut Vec<u32>, start: usize, length: usize, divisor: u32) {
        let bucket_count = self.bucket_count;
        let mut count = Vec::new();
        let mut offset = Vec::new();
        let mut digit;

        for _ in 0..bucket_count {
            count.push(0_u32);
            offset.push(0_u32)
        }

        for i in start..length {
            self.sort_info.read(i);
            let d = data[i];
            let digit = self.get_digit(d, divisor);

            self.sort_info.do_nothing();
            count[digit as usize] += 1;
        }

        offset[0] = start as u32;

        for i in 1..bucket_count as usize {
            self.sort_info.do_nothing();
            self.sort_info.do_nothing();
            self.sort_info.do_nothing();
            offset[i] = count[i-1] + offset[i-1];
        }

        for b in 0..bucket_count as usize {
            while count[b] > 0 {
                self.sort_info.do_nothing();
                let origin = offset[b];
                let mut from = origin;
                self.sort_info.do_nothing();
                let mut num = data[from as usize];

                loop {
                    digit = self.get_digit(num, divisor);
                    self.sort_info.do_nothing();
                    let to = offset[digit as usize];

                    self.sort_info.do_nothing();
                    offset[digit as usize] += 1;
                    self.sort_info.do_nothing();
                    count[digit as usize] -= 1;

                    let tmp = data[to as usize];
                    self.sort_info.write(data, to as usize, num);

                    num = tmp;
                    from = to;

                    if from == origin {
                        break;
                    }
                }
            }
        }

        if divisor > 1 {
            for i in 0..bucket_count as usize {
                let begin = match i > 0 {
                    true => {
                        self.sort_info.do_nothing();
                        offset[i-1]
                    },
                    false => start as u32
                };

                self.sort_info.do_nothing();
                let end = offset[i];

                if end - begin > 1 {
                    self.sort(data, begin as usize, end as usize, divisor / self.bucket_count);
                }
            }
        }
    }
}