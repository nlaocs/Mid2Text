pub fn tick_to_string(ticks: u32) -> String {
    let mut result = String::new();
    if ticks == 0 {
        return "".to_string();
    }
    let mut remaining_ticks = ticks;

    while remaining_ticks >= 18 {
        result.push('9');
        remaining_ticks -= 18;
    }

    if remaining_ticks >= 2 {
        let digit = remaining_ticks / 2;
        result.push_str(&digit.to_string());
        remaining_ticks %= 2;
    }

    if remaining_ticks == 1 {
        result.push('.');
    }

    if result.is_empty() {
        result.push('.');
    }

    result
}

pub fn merge_string(tracks: &Vec<String>) -> String {
    fn parse_track(track: &str) -> Vec<(u32, char)> {
        let mut events = Vec::new();
        let mut current_tick = 0;
        let mut chars = track.chars();

        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                current_tick += digit * 2;
            } else if c == '.' {
                current_tick += 1;
            } else {
                events.push((current_tick, c));
            }
        }

        events
    }

    fn serialize_events(events: &[(u32, char)]) -> String {
        if events.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let mut last_tick = 0;
        let mut i = 0;

        while i < events.len() {
            let current_tick = events[i].0;

            if current_tick > last_tick {
                result.push_str(tick_to_string(current_tick - last_tick).as_str());
            }

            while i < events.len() && events[i].0 == current_tick {
                result.push(events[i].1);
                i += 1;
            }

            last_tick = current_tick;
        }

        result
    }

    let mut all_events = Vec::new();

    for track in tracks {
        let events = parse_track(track);
        all_events.extend(events);
    }

    all_events.sort_by_key(|(tick, _)| *tick);

    serialize_events(&all_events)
}

#[test]
fn test_tick_conversion() {
    assert_eq!(tick_to_string(0), "".to_string());
    assert_eq!(tick_to_string(1), ".".to_string());
    assert_eq!(tick_to_string(2), "1".to_string());
    assert_eq!(tick_to_string(3), "1.".to_string());
    assert_eq!(tick_to_string(4), "2".to_string());

    assert_eq!(tick_to_string(38), "991".to_string());
}

#[test]
fn test_merge_string() {
    assert_eq!(
        merge_string(&vec!["G4I4K".to_string(), "2G4I4K".to_string()]),
        "G2G2I2I2K2K".to_string()
    );

    assert_eq!(
        merge_string(&vec!["G4I4K".to_string(), "2G4I4KA".to_string()]),
        "G2G2I2I2K2KA".to_string()
    );

    assert_eq!(
        merge_string(&vec!["AA@.A".to_string(), "BB1B".to_string(), "CCC".to_string()]),
        "AA@BBCCC.A.B".to_string()
    );

    assert_eq!(
        merge_string(&vec!["=G9999999999999999999999999999999999999999999999997=G6=G3=G96=G9=G96=G9=G96=G".to_string(), "B1B".to_string()]),
        "=GB1B9999999999999999999999999999999999999999999999996=G6=G3=G96=G9=G96=G9=G96=G".to_string()
    );
}
