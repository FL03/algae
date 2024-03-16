/*
    Appellation: partial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Layer;

pub struct MerkleLayer {}

pub struct PartialTree {
    layers: Vec<Layer>,
}

impl PartialTree {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }
}
