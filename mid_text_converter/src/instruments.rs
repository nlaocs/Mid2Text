use midly::num::u7;

#[derive(Debug)]
pub struct Note {
    pub key: u7,
    pub start_timing: u32,
}