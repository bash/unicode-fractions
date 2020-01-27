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
        '-' => '\u{207B}',
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
    }
);

transformation!(
    Subscript {
        '-' => '\u{208B}',
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
    }
);
