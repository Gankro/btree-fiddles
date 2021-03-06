// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(unsafe_destructor, unboxed_closures)]
#![feature(core, alloc, hash, test, rand, collections)]

#![allow(deprecated, dead_code, unused_features)]

extern crate test;

mod node;
#[macro_use]
mod bench;
pub mod map;
pub mod set;
