use self::transformation::{Subscript, Superscript, Transformation};
use digits_iterator::DigitsExtension;
use single_character_fractions::find_single_character_fraction;
use std::fmt::{self, Display, Write};

mod single_character_fractions;
mod transformation;

/// Represents a [Vulgar Fraction] that can be formatted to a unicode fraction using the [`Display`] trait.
/// When available, [single character fractions] are used.
///
/// ## Examples
/// ```
/// use vulgar_fractions::VulgarFraction;
/// println!("{}", VulgarFraction::new(10, 3)); // Prints '¹⁰⁄₃'
/// println!("{}", VulgarFraction::new(1, 4)); // Prints '¼'
/// ```
///
/// [Vulgar Fraction]: https://en.wikipedia.org/wiki/Fraction_(mathematics)#Simple,_common,_or_vulgar_fractions
/// [single character fractions]: http://unicodefractions.com
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
        match find_single_character_fraction(self.nominator, self.denominator) {
            Some(fraction) => f.write_char(fraction),
            None => self.write_vulgar_fraction(f),
        }
    }
}

impl VulgarFraction {
    fn write_vulgar_fraction(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const FRACTION_SLASH: char = '\u{2044}';
        write_number::<Superscript>(f, self.nominator)?;
        f.write_char(FRACTION_SLASH)?;
        write_number::<Subscript>(f, self.denominator)?;
        Ok(())
    }
}

fn write_number<T>(f: &mut fmt::Formatter<'_>, number: i64) -> fmt::Result
where
    T: Transformation,
{
    if number.is_negative() {
        f.write_char(T::MINUS_CHARACTER)?;
    }
    for digit in number.abs().digits() {
        write!(f, "{}", T::map_digit(digit))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::single_character_fractions::SINGLE_CHARACTER_FRACTIONS;
    use super::*;
    use std::string::ToString;

    #[test]
    fn all_digits_in_the_nominator_can_be_formatted() {
        assert_eq!(
            "¹²³⁴⁵⁶⁷⁸⁹⁰⁄₁",
            VulgarFraction::new(1_234_567_890, 1).to_string()
        );
    }

    #[test]
    fn all_digits_in_the_denominator_can_be_formatted() {
        assert_eq!(
            "¹⁄₁₂₃₄₅₆₇₈₉₀",
            VulgarFraction::new(1, 1_234_567_890).to_string()
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
        for ((nominator, denominator), expected_fraction) in SINGLE_CHARACTER_FRACTIONS {
            assert_eq!(
                expected_fraction.to_string(),
                VulgarFraction::new(*nominator, *denominator).to_string(),
            );
        }
    }
}
