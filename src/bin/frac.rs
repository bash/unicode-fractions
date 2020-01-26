use std::env::args;
use std::process::exit;
use vulgar_fractions::VulgarFraction;

fn main() {
    match parse_arguments() {
        Ok((nominator, denominator)) => print_fraction(nominator, denominator),
        Err(_) => print_usage_and_abort(),
    }
}

fn print_fraction(nominator: i64, denominator: i64) {
    print!("{}", VulgarFraction::new(nominator, denominator));
}

fn print_usage() {
    let program = args().nth(0).unwrap();
    println!("usage: {} nominator denominator", program);
}

fn print_usage_and_abort() {
    print_usage();
    exit(1);
}

fn parse_arguments() -> Result<(i64, i64), ()> {
    let nominator: i64 = read_nth_argument_as_i64(1)?;
    let denominator: i64 = read_nth_argument_as_i64(2)?;
    Ok((nominator, denominator))
}

fn read_nth_argument_as_i64(n: usize) -> Result<i64, ()> {
    let arg = args().nth(n).ok_or(())?;
    arg.parse().map_err(|_| ())
}
