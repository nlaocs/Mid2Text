use crate::instruments::Instruments;
use crate::utils;

pub struct Song {
    pub tracks: Vec<Instruments>,
    pub end: u32,
}

impl Song {
    pub fn new() -> Self {
        Self { tracks: Vec::new(), end: 0 }
    }

    pub fn add_track(&mut self, track: Instruments) {
        self.tracks.push(track);
    }

    pub fn to_text(&self, relative_move: bool) -> Result<String, Box<dyn std::error::Error>> {
        let mut t = Vec::new();
        for track in &self.tracks {
            t.push(track.to_text(relative_move)?);
        }
        Ok(utils::merge_string(&t))
        
    }
}

impl Default for Song {
    fn default() -> Self {
        Song::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::note::Note;
    use midly::num::u7;
    use crate::instruments::{InstrumentKind, Instruments, Track};
    use crate::song::Song;

    #[test]
    fn test_song() {
        let mut song = Song::new();

        let mut track1 = Track::new();
        track1.push(Note::new(u7::from(60), 0));
        track1.push(Note::new(u7::from(62), 1));

        let mut track2 = Track::new();
        track2.push(Note::new(u7::from(112), 2));
        track2.push(Note::new(u7::from(125), 3));

        let instrument1 = Instruments::new(InstrumentKind::Flute, track1);
        let instrument2 = Instruments::new(InstrumentKind::Pling, track2);

        song.add_track(instrument1);
        song.add_track(instrument2);
        let result = song.to_text(true).unwrap();
        assert_eq!(result, "@G.@I.+W.+X");
    }
}
pub mod mid {
    use crate::instruments::Track;

    pub fn mid_to_track(path: &str) -> Result<Track, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        let smf = midly::Smf::parse(&data)?;
        let track = Track::midi_to_track(&smf);
        Ok(track)

    }

    #[cfg(test)]
    mod tests {
        use crate::note::Note;
        use midly::num::u7;
        use crate::instruments::Track;
        use crate::song::mid::mid_to_track;

        #[test]
        fn test_mid_to_track() {
            let track = mid_to_track("./one_octave.mid").unwrap();

            let mut track2 = Track::new();
            track2.push(Note::new(u7::from(60), 0));
            track2.push(Note::new(u7::from(62), 8));
            track2.push(Note::new(u7::from(64), 16));
            track2.push(Note::new(u7::from(65), 24));
            track2.push(Note::new(u7::from(67), 32));
            track2.push(Note::new(u7::from(69), 40));
            track2.push(Note::new(u7::from(71), 48));
            track2.push(Note::new(u7::from(72), 56));

            assert_eq!(track.len(), 8);
            assert_eq!(track, track2);
        }
    }
}
