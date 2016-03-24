// Copyright 2015-2016 textnonce Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![cfg_attr(not(feature = "with-syntex"), feature(custom_attribute, custom_derive, plugin))]
#![cfg_attr(not(feature = "with-syntex"), plugin(serde_macros))]

extern crate rand;
extern crate time;
extern crate rustc_serialize;
extern crate serde;


#[cfg(not(feature = "with-syntex"))]
include!("lib.rs.in");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));


