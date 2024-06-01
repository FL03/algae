/*
    Appellation: spline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::error::*;

pub struct BSpline<C, K> {
    pub(crate) degree: usize,
    pub(crate) ctrls: Vec<C>,
    pub(crate) knots: Vec<K>,
}

impl<C, K> BSpline<C, K> {
    pub fn new(degree: usize, ctrls: Vec<C>, knots: Vec<K>) -> Result<Self> {
        if ctrls.len() < degree {
            return Err(SplineError::NotEnoughPoints);
        }
        if knots.len() != ctrls.len() + degree + 1 {
            return Err(SplineError::not_enough_knots(knots.len(), ctrls.len() + degree + 1));
        }
        let spline = Self {
            ctrls,
            degree,
            knots,
        };
        Ok(spline)
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn knots(&self) -> &[K] {
        &self.knots
    }

    pub fn knots_mut(&mut self) -> &mut [K] {
        &mut self.knots
    }

    pub fn points(&self) -> &[C] {
        &self.ctrls
    }

    pub fn points_mut(&mut self) -> &mut [C] {
        &mut self.ctrls
    }
}
