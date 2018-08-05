#![feature(test)]

extern crate spork;
extern crate test;

use spork::{Spork, StatType};

#[bench]
fn bench_get_stats(b: &mut ::test::Bencher) {
    // darwin: 2,801 ns/iter (+/- 379)
    // linux: 241,176 ns/iter (+/- 2,009)
    // windows: 478 ns/iter (+/- 10)
    let spork = Spork::new().unwrap();

    b.iter(move || {
        spork.stats(StatType::Process).unwrap();
    })
}