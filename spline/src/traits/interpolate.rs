/*
    Appellation: interpolate <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::ops::Mul;
use num::traits::{MulAdd, Num, NumRef};

pub trait Interpolate<A, B> {
    type Output;

    fn interpolate(&self, data: A, t: B) -> Self::Output;
}

/*
 ************* Implementations *************
*/
impl<A, B, C, T> Interpolate<A, B> for T
where
    A: Num + Mul<B, Output = C>,
    B: Clone + Num + NumRef,
    T: Clone + MulAdd<B, C> + Num,
{
    type Output = <T as MulAdd<B, C>>::Output;

    fn interpolate(&self, data: A, t: B) -> Self::Output {
        // *self * (B::one() - t) + *data * t
        self.clone().mul_add(B::one() - t.clone(), data * t)
    }
}
