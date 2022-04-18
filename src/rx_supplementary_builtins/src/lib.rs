//! Provides missing compiler library routines
#![no_std]
#![feature(linkage)]
use core::cmp::Ordering;

// FIXME: Find the correct build options to make libgcc emit these subroutines

#[linkage = "weak"]
#[allow(improper_ctypes_definitions)] // compiler builtin
#[no_mangle]
pub extern "C" fn __ucmpti2(a: u128, b: u128) -> i32 {
    let (a, b) = unsafe {
        (
            (*(&a as *const u128 as *const [u32; 4])).iter().rev(),
            (*(&b as *const u128 as *const [u32; 4])).iter().rev(),
        )
    };
    for (&a, &b) in a.zip(b) {
        match a.cmp(&b) {
            Ordering::Greater => return 2,
            Ordering::Equal => {}
            Ordering::Less => return 0,
        }
    }
    1
}

#[linkage = "weak"]
#[allow(improper_ctypes_definitions)] // compiler builtin
#[no_mangle]
pub extern "C" fn __cmpti2(a: i128, b: i128) -> i32 {
    let (mut a, mut b) = unsafe {
        (
            (*(&a as *const i128 as *const [u32; 4])).iter().rev(),
            (*(&b as *const i128 as *const [u32; 4])).iter().rev(),
        )
    };
    {
        let (&a, &b) = (a.next().unwrap(), b.next().unwrap());
        match (a as i32).cmp(&(b as i32)) {
            Ordering::Greater => return 2,
            Ordering::Equal => {}
            Ordering::Less => return 0,
        }
    }
    for (&a, &b) in a.zip(b) {
        match a.cmp(&b) {
            Ordering::Greater => return 2,
            Ordering::Equal => {}
            Ordering::Less => return 0,
        }
    }
    1
}
