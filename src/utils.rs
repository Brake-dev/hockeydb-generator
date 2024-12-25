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
