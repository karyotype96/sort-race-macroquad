#[derive(Clone, Copy, Debug)]
pub struct ReadData {
    pub index: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct SwapData {
    pub index1: usize,
    pub index2: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct WriteData {
    pub index: usize,
    pub from: u32,
    pub to: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum SortMove {
    Read(ReadData),
    Swap(SwapData),
    Write(WriteData),
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

    // move the sort back 1 tick
    fn withdraw_sort(&mut self);

    // returns true if the sort is finished
    fn is_finished(&self) -> bool;

    fn get_name(&self) -> &'static str;
}