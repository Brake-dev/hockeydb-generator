use std::ops::RangeInclusive;

pub fn format_num_to_ordinal(num: i32) -> String {
    let two_digits = num % 100;
    let digit = two_digits % 10;

    if digit == 1 && two_digits != 11 {
        return num.to_string() + "st";
    } else if digit == 2 && two_digits != 12 {
        return num.to_string() + "nd";
    } else if digit == 3 && two_digits != 13 {
        return num.to_string() + "rd";
    }

    return num.to_string() + "th";
}

fn modify_range(range: &RangeInclusive<i32>, line: &i32, position: &String) -> RangeInclusive<i32> {
    let is_defense = position == "D";

    let range_mod = if *line == 1 && !is_defense {
        let start = (*range.start() as f32 * 0.8) as i32;
        start..=*range.end()
    } else if *line == 2 || *line == 1 && is_defense {
        let start = (*range.start() as f32 * 0.5) as i32;
        let end = (*range.end() as f32 * 0.8) as i32;
        start..=end
    } else if *line == 3 || *line == 2 && is_defense {
        let start = (*range.start() as f32 * 0.2) as i32;
        let end = (*range.end() as f32 * 0.5) as i32;
        start..=end
    } else {
        let end = (*range.end() as f32 * 0.2) as i32;
        *range.start()..=end
    };

    let corrected_range_mod = if range_mod.start() > range_mod.end() {
        *range_mod.end()..=*range_mod.start()
    } else {
        range_mod
    };

    corrected_range_mod
}

pub fn adjust_range_by_line_and_pos(
    range: &RangeInclusive<i32>,
    line: &i32,
    position: &String,
) -> RangeInclusive<i32> {
    let range_mod = modify_range(range, line, position);

    if *range_mod.end() < 0 {
        return 0..=*range.end();
    } else {
        return range_mod;
    }
}

pub fn adjust_range_by_line_and_pos_allow_negative(
    range: &RangeInclusive<i32>,
    line: &i32,
    position: &String,
) -> RangeInclusive<i32> {
    modify_range(range, line, position)
}

pub fn adjust_float_range_by_goalie_index(
    range: &RangeInclusive<f32>,
    goalie_index: &usize,
    low_is_better: bool,
) -> RangeInclusive<f32> {
    let adjusted_index = if low_is_better && *goalie_index == 0 {
        &(1 as usize)
    } else if low_is_better && *goalie_index == 1 {
        &(0 as usize)
    } else {
        goalie_index
    };

    let range_mod = if *adjusted_index == 0 {
        let start = *range.start() * 0.8;
        start..=*range.end()
    } else {
        let end = *range.end() * 0.2;
        *range.start()..=end
    };

    let corrected_range_mod = if range_mod.start() > range_mod.end() {
        *range_mod.end()..=*range_mod.start()
    } else {
        range_mod
    };

    if *corrected_range_mod.end() < 0.0 {
        return 0.0..=*range.end();
    } else {
        return corrected_range_mod;
    }
}

pub fn adjust_range_by_line_and_pos_low_is_better(
    range: &RangeInclusive<i32>,
    line: &i32,
    position: &String,
) -> RangeInclusive<i32> {
    let adjusted_line = if *line == 1 {
        4
    } else if *line == 2 {
        3
    } else if *line == 3 {
        2
    } else {
        1
    };

    modify_range(range, &adjusted_line, position)
}
