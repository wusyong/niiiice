use std::os::raw::{c_int, c_char};
use std::slice;

use bee_crypto::{Sponge};
use bee_ternary::{
    T1B1,
    Trits,
};

pub type Kerl = bee_crypto::Kerl;

#[no_mangle]
pub extern "C" fn sponge_new() -> *mut Kerl {
    Box::into_raw(Box::new(Kerl::new()))
}

#[no_mangle]
pub extern "C" fn sponge_free(ptr: *mut Kerl) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn sponge_hash_trytes(ptr: *mut Kerl, trits: *const c_char, length: c_int) -> *const c_char {
    let array: &[c_char] = unsafe {
        assert!(!ptr.is_null());
        slice::from_raw_parts(trits, length as usize)
    };
    let sponge = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let trits = unsafe { Trits::<T1B1>::from_raw_unchecked(array, length as usize) };
    sponge.digest(&trits).unwrap().as_i8_slice().as_ptr()
}
