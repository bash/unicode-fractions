use digits_iterator::DigitsExtension;
use std::fmt::{self, Display, Write};

// Sources:
// - http://unicodefractions.com
// - https://www.unicode.org/charts/PDF/U2150.pdf
// - https://www.unicode.org/charts/PDF/U0080.pdf
static SPECIAL_FRACTIONS: &[((i64, i64), char)] = &[
    ((0, 3), '↉'),
    ((1, 2), '½'),
    ((1, 3), '⅓'),
    ((1, 4), '¼'),
    ((1, 5), '⅕'),
    ((1, 6), '⅙'),
    ((1, 7), '⅐'),
    ((1, 8), '⅛'),
    ((1, 9), '⅑'),
    ((1, 10), '⅒'),
    ((2, 3), '⅔'),
    ((2, 5), '⅖'),
    ((3, 4), '¾'),
    ((3, 5), '⅗'),
    ((3, 8), '⅜'),
    ((4, 5), '⅘'),
    ((5, 6), '⅚'),
    ((5, 8), '⅝'),
    ((7, 8), '⅞'),
];

#[derive(Debug, Copy, Clone)]
pub struct VulgarFraction {
    nominator: i64,
    denominator: i64,
}

impl VulgarFraction {
    pub fn new(nominator: i64, denominator: i64) -> Self {
        Self {
            nominator,
            denominator,
        }
    }
}

impl Display for VulgarFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match find_special_vulgar_fraction(self.nominator, self.denominator) {
            Some(fraction) => f.write_char(fraction),
            None => self.write_vulgar_fraction(f),
        }
    }
}

impl VulgarFraction {
    fn write_vulgar_fraction(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const FRACTION_SLASH: char = '\u{2044}';
        self.write_nominator(f)?;
        f.write_char(FRACTION_SLASH)?;
        self.write_denominator(f)
    }

    fn write_nominator(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const SUPERSCRIPT_MINUS: char = '\u{207B}';
        write_number(f, self.nominator, SUPERSCRIPT_MINUS, digit_to_superscript)
    }

    fn write_denominator(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const SUBSCRIPT_MINUS: char = '\u{208B}';
        write_number(f, self.denominator, SUBSCRIPT_MINUS, digit_to_subscript)
    }
}

fn write_number(
    f: &mut fmt::Formatter<'_>,
    number: i64,
    minus_character: char,
    map_digit: impl Fn(u8) -> char,
) -> fmt::Result {
    if number.is_negative() {
        f.write_char(minus_character)?;
    }
    for digit in number.abs().digits() {
        write!(f, "{}", map_digit(digit))?;
    }
    Ok(())
}

fn find_special_vulgar_fraction(nominator: i64, denominator: i64) -> Option<char> {
    let index = SPECIAL_FRACTIONS
        .binary_search_by_key(&(nominator, denominator), |entry| entry.0)
        .ok()?;
    Some(SPECIAL_FRACTIONS[index].1)
}

fn digit_to_superscript(digit: u8) -> char {
    match digit {
        0 => '\u{2070}',
        1 => '\u{00B9}',
        2 => '\u{00B2}',
        3 => '\u{00B3}',
        4 => '\u{2074}',
        5 => '\u{2075}',
        6 => '\u{2076}',
        7 => '\u{2077}',
        8 => '\u{2078}',
        9 => '\u{2079}',
        _ => unreachable!(),
    }
}

fn digit_to_subscript(digit: u8) -> char {
    match digit {
        0 => '\u{2080}',
        1 => '\u{2081}',
        2 => '\u{2082}',
        3 => '\u{2083}',
        4 => '\u{2084}',
        5 => '\u{2085}',
        6 => '\u{2086}',
        7 => '\u{2087}',
        8 => '\u{2088}',
        9 => '\u{2089}',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn all_digits_in_the_nominator_can_be_formatted() {
        assert_eq!(
            "¹²³⁴⁵⁶⁷⁸⁹⁰⁄₁",
            VulgarFraction::new(1234567890, 1).to_string()
        );
    }

    #[test]
    fn all_digits_in_the_denominator_can_be_formatted() {
        assert_eq!(
            "¹⁄₁₂₃₄₅₆₇₈₉₀",
            VulgarFraction::new(1, 1234567890).to_string()
        );
    }

    #[test]
    fn negative_nominators_are_formatted_correctly() {
        assert_eq!("⁻⁵⁄₄", VulgarFraction::new(-5, 4).to_string());
    }

    #[test]
    fn negative_denominators_are_formatted_correctly() {
        assert_eq!("⁵⁄₋₄", VulgarFraction::new(5, -4).to_string());
    }

    #[test]
    fn fraction_with_negative_nominator_and_negative_denominator_is_formatted_correctly() {
        assert_eq!("⁻⁵⁄₋₄", VulgarFraction::new(-5, -4).to_string());
    }

    #[test]
    fn single_character_fractions_are_used() {
        for ((nominator, denominator), expected_fraction) in SPECIAL_FRACTIONS {
            assert_eq!(
                expected_fraction.to_string(),
                VulgarFraction::new(*nominator, *denominator).to_string(),
            );
        }
    }

    #[test]
    fn special_fractions_are_sorted_by_the_key_so_that_binary_searching_works() {
        let mut sorted = SPECIAL_FRACTIONS.to_vec();
        sorted.sort_by_key(|entry| entry.0);
        assert_eq!(sorted, SPECIAL_FRACTIONS);
    }
}
