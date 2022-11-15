/*
    Appellation: mmr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::{nodes::MerkleNode, is_node_right, sibling_index};
use digest::Digest;
use scsys::prelude::{H256, Hashable};
use std::{collections::HashMap, marker::PhantomData};


pub struct MerkleMountainRange<T, D>
where
    T: Hashable,
    D: Digest,
{
    // todo convert these to a bitmap
    mmr: Vec<MerkleNode>,
    data: HashMap<H256, T>,
    hasher: PhantomData<D>,
    current_peak_height: (usize, usize), // we store a tuple of peak height,index
}

impl<T, D> MerkleMountainRange<T, D>
where
    T: Hashable,
    D: Digest,
{
    /// This function creates a new empty Merkle Mountain Range
    pub fn new() -> MerkleMountainRange<T, D> {
        MerkleMountainRange {
            mmr: Vec::new(),
            data: HashMap::new(),
            hasher: PhantomData,
            current_peak_height: (0, 0),
        }
    }

    /// This function returns a reference to the data stored in the mmr
    /// It will return none if the hash does not exist
    pub fn get_object(&self, hash: &H256) -> Option<&T> {
        self.data.get(hash)
    }

    /// This function returns a mut reference to the data stored in the MMR
    /// It will return none if the hash does not exist
    pub fn get_mut_object(&mut self, hash: &H256) -> Option<&mut T> {
        self.data.get_mut(hash)
    }

    pub fn get_hash(&self, index: usize) -> Option<H256> {
        if index > self.get_last_added_index() {
            return None;
        };
        Some(self.mmr[index].hash.clone())
    }

    /// This function returns the hash proof tree of a given hash.
    /// If the given hash is not in the tree, the vec will be empty.
    /// The Vec will be created in form of the Lchild-Rchild-parent(Lchild)-Rchild-parent-..
    /// This pattern will be repeated until the parent is the root of the MMR
    pub fn get_hash_proof(&self, hash: &H256) -> Vec<H256> {
        let mut result = Vec::new();
        let mut i = self.mmr.len();
        for counter in 0..self.mmr.len() {
            if self.mmr[counter].hash == *hash {
                i = counter;
                break;
            }
        }
        if i == self.mmr.len() {
            return result;
        };
        self.get_ordered_hash_proof(i, &mut result);

        if self.current_peak_height.1 == self.get_last_added_index() {
            // we know there is no bagging as the mmr is a balanced binary tree
            return result;
        }

        let mut peaks = self.bag_mmr();
        let mut i = peaks.len();
        let mut was_on_correct_height = false;
        while i > 1 {
            // was_on_correct_height is used to track should we add from this point onwards both left and right
            // siblings. This loop tracks from bottom of the peaks, so we keep going up until we hit a known
            // point, we then add the missing sibling from that point
            if was_on_correct_height {
                result.push(peaks[i - 2].clone());
                result.push(peaks[i - 1].clone());
            } else if peaks[i - 1] == result[result.len() - 1] {
                result.insert(result.len() - 1, peaks[i - 2].clone());
                was_on_correct_height = true;
            } else if peaks[i - 2] == result[result.len() - 1] {
                result.push(peaks[i - 1].clone());
                was_on_correct_height = true;
            }

            let mut hasher = D::new();
            hasher.input(&peaks[i - 2]);
            hasher.input(&peaks[i - 1]);
            peaks[i - 2] = hasher.result().to_vec();
            i -= 1;
        }
        // lets calculate the final new peak
        let mut hasher = D::new();
        hasher.input(&self.mmr[self.current_peak_height.1].hash);
        hasher.input(&peaks[0]);
        if was_on_correct_height {
            // edge case, our node is in the largest peak, we have already added it
            result.push(self.mmr[self.current_peak_height.1].hash.clone());
        }
        result.push(peaks[0].clone());
        result.push(hasher.result().to_vec());

        result
    }

    // This function is an iterative function. It will add the left node first then the right node to the provided array
    // on the index. It will return when it reaches a single highest point.
    // this function will return the index of the local peak, negating the need to search for it again.
    fn get_ordered_hash_proof(&self, index: usize, results: &mut Vec<H256>) {
        let sibling = sibling_index(index);
        let mut next_index = index + 1;
        if sibling >= self.mmr.len() {
            // we are at a peak
            results.push(self.mmr[index].hash.clone());
            return;
        }
        if sibling < index {
            results.push(self.mmr[sibling].hash.clone());
            results.push(self.mmr[index].hash.clone());
        } else {
            results.push(self.mmr[index].hash.clone());
            results.push(self.mmr[sibling].hash.clone());
            next_index = sibling + 1;
        }
        self.get_ordered_hash_proof(next_index, results);
    }

    /// This function will verify the provided proof. Internally it uses the get_hash_proof function to construct a
    /// similar proof. This function will return true if the proof is valid
    /// If the order does not match Lchild-Rchild-parent(Lchild)-Rchild-parent-.. the validation will fail
    /// This function will only succeed if the given hash is of height 0
    pub fn verify_proof(&self, hashes: &Vec<H256>) -> bool {
        if hashes.len() == 0 {
            return false;
        }
        if self.get_object(&hashes[0]).is_none() && self.get_object(&hashes[1]).is_none() {
            // we only want to search for valid object's proofs, either 0 or 1 must be a valid object
            return false;
        }
        let proof = self.get_hash_proof(&hashes[0]);
        hashes.eq(&proof)
    }

    // This function calculates the peak height of the mmr
    fn calc_peak_height(&self) -> (usize, usize) {
        let mut height_counter = 0;
        let mmr_len = self.get_last_added_index();
        let mut index: usize = (1 << height_counter + 2) - 2;
        let mut actual_height_index = 0;
        while mmr_len >= index {
            // find the height of the tree by finding if we can subtract the  height +1
            height_counter += 1;
            actual_height_index = index;
            index = (1 << height_counter + 2) - 2;
        }
        (height_counter, actual_height_index)
    }

    /// This function returns the peak height of the mmr
    pub fn get_peak_height(&self) -> usize {
        self.current_peak_height.0
    }

    /// This function will return the single merkle root of the MMR.
    pub fn get_merkle_root(&self) -> H256 {
        let mut peaks = self.bag_mmr();
        let mut i = peaks.len();
        while i > 1 {
            // lets bag all the other peaks
            let mut hasher = D::new();
            hasher.input(&peaks[i - 2]);
            hasher.input(&peaks[i - 1]);
            peaks[i - 2] = hasher.result().to_vec();
            i -= 1;
        }
        if peaks.len() > 0 {
            // if there was other peaks, lets bag them with the highest peak
            let mut hasher = D::new();
            hasher.input(&self.mmr[self.current_peak_height.1].hash);
            hasher.input(&peaks[0]);
            return hasher.result().to_vec();
        }
        // there was no other peaks, return the highest peak
        return self.mmr[self.current_peak_height.1].hash.clone();
    }

    /// This function adds a vec of leaf nodes to the mmr.
    pub fn add_vec(&mut self, objects: Vec<T>) {
        for object in objects {
            self.add_single(object);
        }
    }

    /// This function adds a new leaf node to the mmr.
    pub fn add_single(&mut self, object: T) {
        let node_hash = object.hash();
        let node = MerkleNode::from(node_hash.clone());
        self.data.insert(node_hash, object);
        self.mmr.push(node);
        if is_node_right(self.get_last_added_index()) {
            self.add_single_no_leaf(self.get_last_added_index())
        }
    }

    // This function adds non leaf nodes, eg nodes that are not directly a hash of data
    // This is iterative and will continue to up and till it hits the top, will be a future left child
    fn add_single_no_leaf(&mut self, index: usize) {
        let mut hasher = D::new();
        hasher.input(&self.mmr[sibling_index(index)].hash);
        hasher.input(&self.mmr[index].hash);
        let new_hash = hasher.result().to_vec();
        let new_node = MerkleNode::from(new_hash);
        self.mmr.push(new_node);
        if is_node_right(self.get_last_added_index()) {
            self.add_single_no_leaf(self.get_last_added_index())
        } else {
            self.current_peak_height = self.calc_peak_height(); // because we have now stopped adding right nodes, we need to update the height of the mmr
        }
    }

    // This function is just a private function to return the index of the last added node
    fn get_last_added_index(&self) -> usize {
        self.mmr.len() - 1
    }

    fn bag_mmr(&self) -> Vec<H256> {
        // lets find all peaks of the mmr
        let mut peaks = Vec::new();
        self.find_bagging_indexes(
            self.current_peak_height.0 as i64,
            self.current_peak_height.1,
            &mut peaks,
        );
        peaks
    }

    fn find_bagging_indexes(&self, mut height: i64, index: usize, peaks: &mut Vec<H256>) {
        let mut new_index = index + (1 << height + 1) - 1; // go the potential right sibling
        while (new_index > self.get_last_added_index()) && (height > 0) {
            // lets go down left child till we hit a valid node or we reach height 0
            new_index = new_index - (1 << height);
            height -= 1;
        }
        if (new_index <= self.get_last_added_index()) && (height >= 0) {
            // is this a valid peak which needs to be bagged
            peaks.push(self.mmr[new_index].hash.clone());
            self.find_bagging_indexes(height, new_index, peaks); // lets go look for more peaks
        }
    }
}
