use midly::num::u7;

#[derive(Debug)]
pub struct Note {
    pub key: u7,
    pub start_timing: u32,
}

impl Note {
    pub fn new(key: u7, start_timing: u32) -> Self {
        Self { key, start_timing }
    }

    pub fn key_to_char(key: u8) -> Option<char> {
        if key < 54 && key > 78 {
            // todo error
            return None;
        };
        let mut k = key;
        k += 11;
        let char = char::from(k);
        Some(char)
    }

    pub fn to_char(&self) -> char {
        Self::key_to_char(self.key.as_int()).unwrap()
    }
}

