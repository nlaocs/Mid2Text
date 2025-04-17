use crate::note::{Note};
use std::fmt::Write;
use std::mem;
use midly::num::u7;
use thiserror::Error;
use crate::utils;

#[derive(Debug, Clone, PartialEq)]
pub struct Track(pub Vec<Note>);

impl Track {
    pub fn new() -> Self {
        Track(Vec::new())
    }

    pub fn merge(&mut self, mut track: Self) {
        self.0.append(&mut track.0);
        self.0.sort_by(|a, b| a.start_timing.cmp(&b.start_timing));
    }

    pub fn midi_to_notes(midi: &midly::Smf) -> Self {
        let tracks: Vec<Vec<&midly::TrackEvent>> = midi.tracks
            .iter()
            .map(|track| {
                track.iter()
                    .filter(|event| matches!(
                    event.kind,
                    midly::TrackEventKind::Midi {
                        message: midly::MidiMessage::NoteOn { .. }
                        | midly::MidiMessage::NoteOff { .. },
                        ..
                    }
                ))
                    .collect()
            })
            .collect();

        let mut notes = Track::new();

        for track in tracks {
            if track.is_empty() {
                continue;
            };
            let mut buff_tracks = track.clone();
            let mut t = Track::new();
            let mut current_time = 0u32;

            for track in &track {
                current_time += track.delta.as_int();
                match &track.kind {
                    midly::TrackEventKind::Midi {
                        message: midly::MidiMessage::NoteOn { key, .. },
                        ..
                    } => {
                        if let Some(index) = buff_tracks.iter().position(|t| {
                            if let midly::TrackEventKind::Midi {
                                message: midly::MidiMessage::NoteOff { key: k, .. },
                                ..
                            } = &t.kind
                            {
                                key == k
                            } else {
                                false
                            }
                        }) {
                            let note = Note::new(*key, current_time / 12);
                            t.0.push(note);
                            buff_tracks.remove(index);
                        };
                    }
                    _ => {}
                }
            }

            if !t.0.is_empty() {
                notes.merge(t);
            }
        }

        notes
    }
}

pub enum InstrumentKind {
    Pling,
    Hat,
    Snare,
    BassDrum,
    Bass,
    Bell,
    Chime,
    Flute,
    Guitar,
    Harp,
    Xylophone,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum InstrumentError {
    #[error("Merge Different Instrument Types")]
    MergeDifferentInstrumentTypes,
}

pub enum Instruments {
    Pling(Track),
    Hat(Track),
    Snare(Track),
    BassDrum(Track),
    Bass(Track),
    Bell(Track),
    Chime(Track),
    Flute(Track),
    Guitar(Track),
    Harp(Track),
    Xylophone(Track),
}

impl Instruments {
    pub fn new(instrument_kind: InstrumentKind, track: Track) -> Self {
        match instrument_kind {
            InstrumentKind::Pling => Instruments::Pling(track),
            InstrumentKind::Hat => Instruments::Hat(track),
            InstrumentKind::Snare => Instruments::Snare(track),
            InstrumentKind::BassDrum => Instruments::BassDrum(track),
            InstrumentKind::Bass => Instruments::Bass(track),
            InstrumentKind::Bell => Instruments::Bell(track),
            InstrumentKind::Chime => Instruments::Chime(track),
            InstrumentKind::Flute => Instruments::Flute(track),
            InstrumentKind::Guitar => Instruments::Guitar(track),
            InstrumentKind::Harp => Instruments::Harp(track),
            InstrumentKind::Xylophone => Instruments::Xylophone(track),
        }
    }

    /// Merges two tracks of the same instrument type.
    pub fn merge(&mut self, track: Self) -> Result<(), InstrumentError> {
        if mem::discriminant(self) == mem::discriminant(&track) {
            match (self, track) {
                (Instruments::Pling(self_track), Instruments::Pling(mut other_track))
                | (Instruments::Hat(self_track), Instruments::Hat(mut other_track))
                | (Instruments::Snare(self_track), Instruments::Snare(mut other_track))
                | (Instruments::BassDrum(self_track), Instruments::BassDrum(mut other_track))
                | (Instruments::Bass(self_track), Instruments::Bass(mut other_track))
                | (Instruments::Bell(self_track), Instruments::Bell(mut other_track))
                | (Instruments::Chime(self_track), Instruments::Chime(mut other_track))
                | (Instruments::Flute(self_track), Instruments::Flute(mut other_track))
                | (Instruments::Guitar(self_track), Instruments::Guitar(mut other_track))
                | (Instruments::Harp(self_track), Instruments::Harp(mut other_track))
                | (Instruments::Xylophone(self_track), Instruments::Xylophone(mut other_track)) => {
                    self_track.0.append(&mut other_track.0);
                    self_track.0.sort_by(|a, b| a.start_timing.cmp(&b.start_timing));
                }
                _ => unreachable!(),
            }
            Ok(())
        } else {
            Err(InstrumentError::MergeDifferentInstrumentTypes)
        }
    }

    pub fn to_text(&self, relative_move: bool) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Instruments::Pling(track) => {
                let mut result = String::new();
                let mut current_time = 0u32;
                for note in &track.0 {
                    let ticks = note.start_timing - current_time;
                    current_time = note.start_timing;
                    write!(&mut result, "{}", utils::tick_to_string(ticks))?;

                    // todo refactor
                    if relative_move {
                        if note.key.as_int() >= 78 { // octave up
                            write!(&mut result, "+{}", Note::key_to_char(note.key.as_int() - 24, relative_move)?)?;
                        } else if note.key.as_int() <= 54 { // octave down
                            write!(&mut result, "-{}", Note::key_to_char(note.key.as_int() + 24, relative_move)?)?;
                        } else {
                            write!(&mut result, "{}", note.to_char(relative_move)?)?;
                        }
                    } else {
                        if note.key.as_int() >= 78 && note.key.as_int() <= 102 { // octave up
                            write!(&mut result, "+{}", Note::key_to_char(note.key.as_int() - 24, relative_move)?)?;
                        }  else if note.key.as_int() <= 54 && note.key.as_int() >= 30 { // octave down
                            write!(&mut result, "-{}", Note::key_to_char(note.key.as_int() + 24, relative_move)?)?;
                        } else {
                            write!(&mut result, "{}", note.to_char(relative_move)?)?;
                        }
                    }
                }
                Ok(result)
            },
            Instruments::Hat(track) => Instruments::track_to_string(track, Some("!"), relative_move),
            Instruments::Snare(track) => Instruments::track_to_string(track, Some("?"), relative_move),
            Instruments::BassDrum(track) => Instruments::track_to_string(track, Some("="), relative_move),
            Instruments::Bass(track) => Instruments::track_to_string(track, Some("\\"), relative_move),
            Instruments::Bell(track) => Instruments::track_to_string(track, Some("/"), relative_move),
            Instruments::Chime(track) => Instruments::track_to_string(track, Some("_"), relative_move),
            Instruments::Flute(track) => Instruments::track_to_string(track, Some("@"), relative_move),
            Instruments::Guitar(track) => Instruments::track_to_string(track, Some(":"), relative_move),
            Instruments::Harp(track) => Instruments::track_to_string(track, Some(";"), relative_move),
            Instruments::Xylophone(track) => Instruments::track_to_string(track, Some(","), relative_move),
        }
    }

    pub fn track_to_string(track: &Track, prefix: Option<&str>, relative_move: bool) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        let mut current_time = 0u32;
        for note in &track.0 {
            let ticks = note.start_timing - current_time;
            current_time = note.start_timing;
            write!(&mut result, "{}", utils::tick_to_string(ticks))?;

            let ch = note.to_char(relative_move)?;
            if let Some(p) = prefix {
                write!(&mut result, "{}{}", p, ch)?;
            } else {
                write!(&mut result, "{}", ch)?;
            }
        };
        Ok(result)
    }

}

#[test]
fn test_instrument_to_text() {
    let mut track = Track::new();
    track.0.push(Note::new(u7::from(60), 0));
    track.0.push(Note::new(u7::from(62), 1));
    track.0.push(Note::new(u7::from(127), 2));

    let mut instrument = Instruments::new(InstrumentKind::Pling, track);
    let result = instrument.to_text(true).unwrap();
    assert_eq!(result, "G.I.+N");
}

#[test]
fn test_merge_instruments() {
    let mut track1 = Track::new();
    track1.0.push(Note::new(u7::from(60), 0));
    track1.0.push(Note::new(u7::from(62), 1));

    let mut track2 = Track::new();
    track2.0.push(Note::new(u7::from(64), 2));
    track2.0.push(Note::new(u7::from(113), 3));

    let mut instrument1 = Instruments::new(InstrumentKind::Pling, track1);
    let instrument2 = Instruments::new(InstrumentKind::Pling, track2);

    instrument1.merge(instrument2).unwrap();

    let result = instrument1.to_text(true).unwrap();
    assert_eq!(result, "G.I.K.+X");

    let mut track1 = Track::new();
    track1.0.push(Note::new(u7::from(60), 0));
    track1.0.push(Note::new(u7::from(62), 1));

    let mut track2 = Track::new();
    track2.0.push(Note::new(u7::from(112), 2));
    track2.0.push(Note::new(u7::from(125), 3));

    let mut instrument1 = Instruments::new(InstrumentKind::Pling, track1);
    let instrument2 = Instruments::new(InstrumentKind::Pling, track2);

    instrument1.merge(instrument2).unwrap();

    let result = instrument1.to_text(true).unwrap();
    assert_eq!(result, "G.I.+W.+X");
}

#[test]
fn test_merge_different_instruments() {
    let mut track1 = Track::new();
    track1.0.push(Note::new(u7::from(60), 0));
    track1.0.push(Note::new(u7::from(62), 1));

    let mut track2 = Track::new();
    track2.0.push(Note::new(u7::from(64), 2));
    track2.0.push(Note::new(u7::from(65), 3));

    let mut instrument1 = Instruments::new(InstrumentKind::Pling, track1);
    let instrument2 = Instruments::new(InstrumentKind::Hat, track2);

    assert!(instrument1.merge(instrument2).is_err());
}
