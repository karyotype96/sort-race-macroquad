#[derive(Copy, Clone, Debug)]
pub enum SortMove {
    Read(usize),
    Swap(usize, usize),
    Write(usize, u32),
}

pub enum SortType {
    Selection,
    Insertion,
    Bubble,
}

pub trait Sort {
    // this will add the list of SortMoves for the sort struct
    fn init_sort(&mut self, data: &Vec<u32>);

    // during playback, this will execute the next SortMove
    fn advance_sort(&mut self);

    // returns true if the sort is finished
    fn is_finished(&self) -> bool;
}