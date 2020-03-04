use std::ffi::CString;
use std::os::raw::c_char;
use vulgar_fractions::VulgarFraction;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn vulgar_fraction(nominator: i32, denominator: i32) -> *mut c_char {
    let vulgar_fraction =
        VulgarFraction::new(i64::from(nominator), i64::from(denominator)).to_string();
    let c_string = CString::new(vulgar_fraction).expect("Fraction should not contain null bytes");
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn vulgar_fraction_len(raw: *mut c_char) -> usize {
    unsafe {
        let string = CString::from_raw(raw);
        let length = string.as_bytes().len();
        string.into_raw();
        length
    }
}

#[no_mangle]
pub extern "C" fn vulgar_fraction_free(raw: *mut c_char) {
    if !raw.is_null() {
        unsafe {
            CString::from_raw(raw);
        }
    }
}
