// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use libc::{c_void, c_int};

use backtrace::{Callback, Context};

struct BacktraceContext {
    addr: *mut c_void,
}

impl Context for BacktraceContext {
    fn ip(&self) -> *mut c_void { self.addr }
    fn symbol_address(&self) -> *mut c_void { self.addr }
}

extern {
    fn backtrace(buf: *mut *mut c_void, sz: c_int) -> c_int;
}

#[inline(never)] // if this is known to be a function call, it can be skipped it
                 // when tracing
pub fn trace(mut cb: &mut Callback) {
    const SIZE: usize = 100;

    let mut buf: [*mut c_void; SIZE];
    let cnt;
    unsafe {
        buf = mem::zeroed();
        cnt = backtrace(buf.as_mut_ptr(), SIZE as c_int);
    }

    for addr in buf[..cnt as usize].iter() {
        let cx = BacktraceContext { addr: *addr };
        if !cb(&cx) {
            return
        }
    }
}
