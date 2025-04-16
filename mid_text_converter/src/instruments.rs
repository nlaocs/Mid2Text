use midly::num::u7;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum NoteError {
    #[error("Invalid key: {0}")]
    InvalidKey(u8),
}

#[derive(Debug)]
pub struct Note {
    pub key: u7,
    pub start_timing: u32,
}

impl Note {
    pub fn new(key: u7, start_timing: u32) -> Self {
        Self { key, start_timing }
    }

    pub fn key_to_char(key: u8, relative_move: bool) -> Result<char, NoteError> {
        let k = if relative_move {
            let mut k = key;
            while k < 54 { k += 12; }
            while k > 78 { k -= 12; }
            k
        } else if key < 54 || key > 78 {
            return Err(NoteError::InvalidKey(key));
        } else {
            key
        };

        Ok(char::from(k + 11))
    }


    pub fn to_char(&self, relative_move: bool) -> Result<char, NoteError> {
        Self::key_to_char(self.key.as_int(), relative_move)
    }
}

#[test]
fn test_key_to_char() {
    assert_eq!(Note::key_to_char(60, true), Ok('G'));
    assert_eq!(Note::key_to_char(72, true), Ok('S'));
    assert_eq!(Note::key_to_char(84, true), Ok('S'));
    assert_eq!(Note::key_to_char(80, true), Ok('O'));
    assert_eq!(Note::key_to_char(1, true), Ok('H'));

    assert_eq!(Note::key_to_char(60, false), Ok('G'));
    assert_eq!(Note::key_to_char(72, false), Ok('S'));
    assert_eq!(Note::key_to_char(84, false), Err(NoteError::InvalidKey(84)));
    assert_eq!(Note::key_to_char(80, false), Err(NoteError::InvalidKey(80)));
    assert_eq!(Note::key_to_char(1, false), Err(NoteError::InvalidKey(1)));
}

#[test]
fn test_to_char() {
    let note = Note::new(u7::from(60), 0);
    assert_eq!(note.to_char(true), Ok('G'));
    assert_eq!(note.to_char(false), Ok('G'));

    let note = Note::new(u7::from(72), 0);
    assert_eq!(note.to_char(true), Ok('S'));
    assert_eq!(note.to_char(false), Ok('S'));

    let note = Note::new(u7::from(84), 0);
    assert_eq!(note.to_char(true), Ok('S'));
    assert_eq!(note.to_char(false), Err(NoteError::InvalidKey(84)));

    let note = Note::new(u7::from(80), 0);
    assert_eq!(note.to_char(true), Ok('O'));
    assert_eq!(note.to_char(false), Err(NoteError::InvalidKey(80)));

    let note = Note::new(u7::from(1), 0);
    assert_eq!(note.to_char(true), Ok('H'));
    assert_eq!(note.to_char(false), Err(NoteError::InvalidKey(1)));
}