#![feature(test)]

extern crate spork;
extern crate test;

use spork::{Spork, StatType};

#[bench]
fn bench_get_stats(b: &mut ::test::Bencher) {
    // darwin: 2,801 ns/iter (+/- 379)
    let spork = Spork::new().unwrap();

    b.iter(move || {
        spork.stats(StatType::Process).unwrap();
    })
}