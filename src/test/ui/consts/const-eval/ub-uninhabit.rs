// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

union Foo {
    a: usize,
    b: Bar,
    c: &'static Bar,
}

#[derive(Copy, Clone)]
enum Bar {}

const BAD_BAD_BAD: Bar = unsafe { Foo { a: 1 }.b };
//~^ ERROR this constant likely exhibits undefined behavior

const BAD_BAD_REF: &Bar = unsafe { Foo { a: 1 }.c };
//~^ ERROR this constant likely exhibits undefined behavior

fn main() {
}
