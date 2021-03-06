// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This checks that a path that cannot be resolved because of an indeterminate import
// does not trigger an ICE.

mod foo {
    pub use self::*; //~ ERROR unresolved
}

fn main() {
    foo::f(); //~ ERROR unresolved
}
