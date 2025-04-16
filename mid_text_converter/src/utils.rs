pub fn tick_to_string(ticks: u32) ->String {
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

#[test]
fn test_tick_conversion() {
    assert_eq!(tick_to_string(0), "".to_string());
    assert_eq!(tick_to_string(1), ".".to_string());
    assert_eq!(tick_to_string(2), "1".to_string());
    assert_eq!(tick_to_string(3), "1.".to_string());
    assert_eq!(tick_to_string(4), "2".to_string());

    assert_eq!(tick_to_string(38), "991".to_string());
}

