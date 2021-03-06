// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BTreeMap;
use std::iter::Iterator;

#[derive(Eq, Hash, Debug, Ord, PartialEq, PartialOrd)]
struct Zst;

fn main() {
    const N: usize = 8;

    for len in 0..N {
        let mut tester = BTreeMap::new();
        assert_eq!(tester.len(), 0);
        for bit in 0..len {
            tester.insert(Zst, ());
        }
        assert_eq!(tester.len(), if len == 0 { 0 } else { 1 });
        assert_eq!(tester.iter().count(), if len == 0 { 0 } else { 1 });
        assert_eq!(tester.get(&Zst).is_some(), len > 0);
        tester.clear();
    }
}
