#![allow(clippy::missing_safety_doc)]

use fmtastic::VulgarFraction;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn vulgar_fraction(nominator: i32, denominator: i32) -> *mut c_char {
    let vulgar_fraction = VulgarFraction::new(nominator, denominator).to_string();
    let c_string = CString::new(vulgar_fraction).expect("Fraction should not contain null bytes");
    c_string.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn vulgar_fraction_len(raw: *mut c_char) -> usize {
    let string = CString::from_raw(raw);
    let length = string.as_bytes().len();
    _ = string.into_raw();
    length
}

#[no_mangle]
pub unsafe extern "C" fn vulgar_fraction_free(raw: *mut c_char) {
    if !raw.is_null() {
        drop(CString::from_raw(raw));
    }
}
