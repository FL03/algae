// lists.rs
#![feature(test)]

extern crate test;
use algae::list::linked::singly::*;
use test::Bencher;

pub const BENCH_SIZE: usize = 10;

#[bench]
fn basic_bench(b: &mut Bencher) {
    let mut list = SinglyLinkedList::new();

    assert_eq!(list.pop(), None);

    b.iter(|| {
        for i in 0..BENCH_SIZE {
            list.push(i);
        }
    });
}
