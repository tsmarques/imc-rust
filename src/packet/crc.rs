use crc16::{State, ARC};

pub fn compute(data: &[u8]) -> State<ARC> {
    let mut crc = State::<ARC>::new();

    crc.update(data);
    crc
}

pub fn compute_from_range(begin: usize, end: usize, data: &[u8]) -> State<ARC> {
    let mut crc = State::<ARC>::new();
    crc.update(&data[begin..end]);

    crc
}

pub fn update_from_range(crc: &mut State<ARC>, begin: usize, end: usize, data: &[u8]) {
    crc.update(&data[begin..end]);
}
