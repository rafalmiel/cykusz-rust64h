// Copyright 2016 Philipp Oppermann. See the README.md
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(allocator)]
#![feature(const_fn)]

#![allocator]
#![no_std]

use spin::Mutex;
use linked_list_allocator::Heap;

extern crate spin;
extern crate linked_list_allocator;
#[macro_use]
extern crate lazy_static;

pub const HEAP_START: usize = 0xfffff80000000000;
pub const HEAP_SIZE: usize = 64 * 4096; // 100 KiB / 25 pages
pub const HEAP_MAX_SIZE: usize = 4096 * 4096; // 4MB

lazy_static! {
    static ref HEAP: Mutex<Heap> = Mutex::new(unsafe {
        Heap::new(HEAP_START, HEAP_SIZE, HEAP_MAX_SIZE)
    });
}

/// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}

extern {
    #[allow(dead_code)]
    fn notify_alloc(addr: *const u8);
    fn notify_dealloc(addr: *const u8);
    fn request_more_mem(from: *const u8, size: usize);
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    let mut heap = HEAP.lock();
    let a = heap.allocate_first_fit(size, align);

    if let Some(addr) = a {
        return addr;
    } else {
        let top = heap.top();
        let req = align_up(size, 0x1000);

        if top + req > heap.max_top() {
            panic!("Out of mem!");
        }

        unsafe {
            request_more_mem(top as *const u8, req);
        }

        heap.extend_last_hole(req);

        return __rust_allocate(size, align);
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
    unsafe { notify_dealloc(ptr as *const u8) };
    unsafe { HEAP.lock().deallocate(ptr, size, align) };
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, size: usize,
    _new_size: usize, _align: usize) -> usize
{
    size
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize,
                                align: usize) -> *mut u8 {
    use core::{ptr, cmp};

    // from: https://github.com/rust-lang/rust/blob/
    //     c66d2380a810c9a2b3dbb4f93a830b101ee49cc2/
    //     src/liballoc_system/lib.rs#L98-L101

    let new_ptr = __rust_allocate(new_size, align);
    unsafe { ptr::copy(ptr, new_ptr, cmp::min(size, new_size)) };
    __rust_deallocate(ptr, size, align);
    new_ptr
}
