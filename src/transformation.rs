//! Source for mappings: https://unicode.org/charts/PDF/U2070.pdf
pub(crate) trait Transformation {
    const MINUS_CHARACTER: char;
    fn map_digit(digit: u8) -> char;
}

macro_rules! transformation {
    ($name: ident { '-' => $minus_character: expr, $($digit: expr => $character: expr,)* }) => {
        pub(crate) struct $name;
        impl Transformation for $name {
            const MINUS_CHARACTER: char = $minus_character;
            fn map_digit(digit: u8) -> char {
                match digit {
                    $($digit => $character,)*
                    _ => unreachable!(),
                }
            }
        }
    };
}

transformation!(
    Superscript {
        '-' => '⁻',
        0 => '⁰',
        1 => '¹',
        2 => '²',
        3 => '³',
        4 => '⁴',
        5 => '⁵',
        6 => '⁶',
        7 => '⁷',
        8 => '⁸',
        9 => '⁹',
    }
);

transformation!(
    Subscript {
        '-' => '₋',
        0 => '₀',
        1 => '₁',
        2 => '₂',
        3 => '₃',
        4 => '₄',
        5 => '₅',
        6 => '₆',
        7 => '₇',
        8 => '₈',
        9 => '₉',
    }
);
