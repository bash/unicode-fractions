use proc_macro2::TokenStream;
use quote::quote;
use std::cmp::PartialOrd;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::error::Error;
use ucd_parse::{
    parse_by_codepoint, Codepoint, UnicodeData, UnicodeDataDecompositionTag, UnicodeDataNumeric,
};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct VulgarFractionCharacter {
    nominator: i64,
    denominator: i64,
    character: char,
}

const UCD_DATA_PATH: &str = "ucd-data";

fn main() -> Result<(), Box<dyn Error>> {
    let unicode_data = parse_by_codepoint::<_, UnicodeData>(UCD_DATA_PATH)?;
    let vulgar_fractions = vulgar_fractions(&unicode_data);
    let generated_function = generate_vulgar_fractions_lookup_function(vulgar_fractions);
    println!("{}", generated_function);
    Ok(())
}

fn generate_vulgar_fractions_lookup_function(
    vulgar_fractions: impl Iterator<Item = VulgarFractionCharacter>,
) -> TokenStream {
    let entries: TokenStream = match_arms_for_vulgar_fraction(vulgar_fractions);
    quote! {
        pub(crate) fn find_single_character_fraction(nominator: i64, denominator: i64) -> Option<char> {
            match (nominator, denominator) {
                #entries
                _ => None,
            }
        }
    }
}

fn match_arms_for_vulgar_fraction(
    vulgar_fractions: impl Iterator<Item = VulgarFractionCharacter>,
) -> TokenStream {
    vulgar_fractions
        .map(match_arm_for_vulgar_fraction)
        .collect()
}

fn match_arm_for_vulgar_fraction(
    VulgarFractionCharacter {
        nominator,
        denominator,
        character,
    }: VulgarFractionCharacter,
) -> TokenStream {
    quote! { (#nominator, #denominator) => Some(#character), }
}

fn vulgar_fractions(
    data_by_codepoint: &BTreeMap<Codepoint, UnicodeData>,
) -> impl Iterator<Item = VulgarFractionCharacter> + '_ {
    data_by_codepoint
        .values()
        .filter(|row| row.decomposition.tag == Some(UnicodeDataDecompositionTag::Fraction))
        .filter(|row| row.decomposition.len >= 3)
        .map(move |row| codepoint_data_to_vulgar_fraction(data_by_codepoint, row))
}

fn codepoint_data_to_vulgar_fraction(
    data_by_codepoint: &BTreeMap<Codepoint, UnicodeData>,
    row: &UnicodeData,
) -> VulgarFractionCharacter {
    let character = row.codepoint.scalar().unwrap();
    let (nominator_codepoints, denominator_codepoints) =
        split_mapping_into_nominator_and_denominator(row);
    let nominator = integer_value_from_codepoints(data_by_codepoint, nominator_codepoints);
    let denominator = integer_value_from_codepoints(data_by_codepoint, denominator_codepoints);
    VulgarFractionCharacter {
        character,
        nominator,
        denominator,
    }
}

fn split_mapping_into_nominator_and_denominator(
    data: &UnicodeData,
) -> (&[Codepoint], &[Codepoint]) {
    let mut parts = split_mapping_by_fraction_slash(data);
    let nominator = parts.next().expect("A fraction must have a nominator");
    let denominator = parts.next().expect("A fraction must have a denominator");
    assert!(
        parts.next().is_none(),
        "A fraction must decompose into a nominator and a denominator"
    );
    (nominator, denominator)
}

fn split_mapping_by_fraction_slash<'a>(
    data: &'a UnicodeData,
) -> impl Iterator<Item = &'a [Codepoint]> + 'a {
    const FRACTION_SLASH: u32 = 0x2044;
    let fraction_slash = Codepoint::from_u32(FRACTION_SLASH).unwrap();
    data.decomposition
        .mapping()
        .split(move |&c| c == fraction_slash)
}

fn integer_value_from_codepoints(
    data_by_codepoint: &BTreeMap<Codepoint, UnicodeData>,
    codepoints: &[Codepoint],
) -> i64 {
    codepoints
        .iter()
        .rev()
        .enumerate()
        .map(|(index, &codepoint)| {
            let value = integer_value_for_codepoint(data_by_codepoint, codepoint)
                .expect("Unable to get integer value for codepoint");
            value * 10_i64.pow(index.try_into().unwrap())
        })
        .sum()
}

fn integer_value_for_codepoint(
    data_by_codepoint: &BTreeMap<Codepoint, UnicodeData>,
    codepoint: Codepoint,
) -> Option<i64> {
    let data = data_by_codepoint.get(&codepoint)?;
    match data.numeric_type_numeric {
        Some(UnicodeDataNumeric::Integer(value)) => Some(value),
        _ => None,
    }
}
