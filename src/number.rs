pub fn to_locale_string(input: f64) -> String {
    let int_part = input.trunc();
    let int_str = int_part.to_string();
    insert_commas(&int_str)
}

fn insert_commas(integer: &str) -> String {
    let rev_int: String = integer
        .chars()
        .rev()
        .collect::<String>()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 3 == 0 && i != 0 {
                format!(",{}", c)
            } else {
                c.to_string()
            }
        })
        .collect();
    rev_int.chars().rev().collect()
}

pub fn format_with_unit(value: f64) -> String {
    let abs_value = value.abs(); // Work with the absolute value for simplicity

    if abs_value >= 1_000_000_000.0 {
        // Billions
        format!("{:.2}B", value / 1_000_000_000.0)
    } else if abs_value >= 1_000_000.0 {
        // Millions
        format!("{:.2}M", value / 1_000_000.0)
    } else if abs_value >= 1_000.0 {
        // Thousands
        format!("{:.2}K", value / 1_000.0)
    } else {
        // No suffix
        format!("{:.2}", value)
    }
}
