/*
   Appellation: default <test>
   Contrib: FL03 <jo3mccain@icloud.com>
*/

fn addition<A, B, C>(a: A, b: B) -> C
where
    A: core::ops::Add<B, Output = C>,
{
    a + b
}

#[test]
fn compiles() {
    assert_eq!(addition(10, 1), 11);
    assert_ne!(addition(9, 1), 9);
}
