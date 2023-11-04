use nix::sys::mman::{mprotect, ProtFlags};
use rand;
use std::alloc::{alloc, dealloc, Layout};
use std::collections::{HashMap, HashSet, LinkedList};
use std::ffi::c_void;
use std::ptr;

#[repr(C)]
struct Registers {
    d8: u64, d9: u64,  d10: u64, d11: u64, d12: u64,
    d13: u64, d14: u64, d15: u64, x19: u64, x20: u64,
    x21: u64, x22: u64, x23: u64, x24: u64, x25: u64,
    x26: u64, x27: u64, x38: u64,

    x30: u64, 
    sp: u64, 
}

impl Registers {
    fn new(sp: u64) -> Self {
        Registers {
            d8: 0, d9: 0,  d10: 0, d11: 0, d12: 0,
	          d13: 0, d14: 0, d15: 0, x19: 0, x20: 0,
	          x21: 0, x22: 0, x23: 0, x24: 0, x25: 0,
	          x26: 0, x27: 0, x38: 0,
            x30: entry_point as u64,
	          sp,
	      }
    }
}
