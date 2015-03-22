// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Run a single benchmark once.  For use with profiling tools.

#![feature(test)]

extern crate test;
extern crate html5ever;
extern crate tendril;

use std::io;
use std::default::Default;

use test::black_box;

use tendril::{ByteTendril, ReadExt};

use html5ever::tokenizer::{TokenSink, Token};
use html5ever::driver::{tokenize_to, one_input};

struct Sink;

impl TokenSink for Sink {
    fn process_token(&mut self, token: Token) {
        // Don't use the token, but make sure we don't get
        // optimized out entirely.
        black_box(token);
    }
}

fn main() {
    let mut input = ByteTendril::new();
    io::stdin().read_to_tendril(&mut input).unwrap();
    let input = input.try_reinterpret().unwrap();

    tokenize_to(Sink, one_input(input), Default::default());
}
