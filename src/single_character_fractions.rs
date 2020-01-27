pub(crate) fn find_single_character_fraction(nominator: i64, denominator: i64) -> Option<char> {
    let index = SINGLE_CHARACTER_FRACTIONS
        .binary_search_by_key(&(nominator, denominator), |entry| entry.0)
        .ok()?;
    Some(SINGLE_CHARACTER_FRACTIONS[index].1)
}

/// Sources:
/// - http://unicodefractions.com
/// - https://www.unicode.org/charts/PDF/U2150.pdf
/// - https://www.unicode.org/charts/PDF/U0080.pdf
pub(crate) const SINGLE_CHARACTER_FRACTIONS: &[((i64, i64), char)] = &[
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_fractions_are_sorted_by_the_key_so_that_binary_searching_works() {
        let mut sorted = SINGLE_CHARACTER_FRACTIONS.to_vec();
        sorted.sort_by_key(|entry| entry.0);
        assert_eq!(sorted, SINGLE_CHARACTER_FRACTIONS);
    }
}
