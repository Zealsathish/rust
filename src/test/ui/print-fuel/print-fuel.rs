// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name="foo"]
#![allow(dead_code)]

// (#55495: The --error-format is to sidestep an issue in our test harness)
// compile-flags: --error-format human -Z print-fuel=foo
// compile-pass

struct S1(u8, u16, u8);
struct S2(u8, u16, u8);
struct S3(u8, u16, u8);

fn main() {
}
