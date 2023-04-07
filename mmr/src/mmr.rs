/*
    Appellation: mmr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::cmp::{MerkleNode, Position};
use crate::{is_node_right, sibling_index, RangeMap};
use decanter::prelude::{hasher, Hashable, H256};
use digest::Digest;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hashable, PartialEq, Serialize)]
pub struct MerkleMountainRange<T = String>
where
    T: ToString,
{
    data: RangeMap<T>,
    mmr: Vec<MerkleNode>, // todo convert these to a bitmap
    position: Position,
}

impl<T> MerkleMountainRange<T>
where
    T: ToString,
{
    pub fn new(mmr: Vec<MerkleNode>, data: RangeMap<T>, position: Position) -> Self {
        Self {
            mmr,
            data,
            position,
        }
    }
    /// This function adds a vec of leaf nodes to the mmr.
    pub fn add_vec<D: Digest>(&mut self, objects: Vec<T>) {
        for object in objects {
            self.add_single::<D>(object);
        }
    }
    /// This function adds a new leaf node to the mmr.
    pub fn add_single<D: Digest>(&mut self, object: T) {
        let node_hash: H256 = hasher(object.to_string()).into();
        let node = MerkleNode::from(node_hash);
        self.data.insert(node_hash, object);
        self.mmr.push(node);
        if is_node_right(self.get_last_added_index()) {
            self.add_single_no_leaf::<D>(self.get_last_added_index())
        }
    }
    // This function adds non leaf nodes, eg nodes that are not directly a hash of data
    // This is iterative and will continue to up and till it hits the top, will be a future left child
    fn add_single_no_leaf<D: Digest>(&mut self, index: usize) {
        let mut hasher = D::new();
        hasher.update(self.mmr[sibling_index(index)].hash);
        hasher.update(self.mmr[index].hash);
        let new_hash: H256 = hasher.finalize().to_vec().into();
        let new_node = MerkleNode::from(new_hash);
        self.mmr.push(new_node);
        if is_node_right(self.get_last_added_index()) {
            self.add_single_no_leaf::<D>(self.get_last_added_index())
        } else {
            self.position = self.calc_peak_height().into(); // because we have now stopped adding right nodes, we need to update the height of the mmr
        }
    }
    fn bag_mmr(&self) -> Vec<H256> {
        // lets find all peaks of the mmr
        let mut peaks = Vec::new();
        self.find_bagging_indexes(self.position.height as i64, self.position.index, &mut peaks);
        peaks
    }
    // This function calculates the peak height of the mmr
    fn calc_peak_height(&self) -> (usize, usize) {
        let mut height_counter = 0;
        let mmr_len = self.get_last_added_index();
        let mut index: usize = (1 << (height_counter + 2)) - 2;
        let mut actual_height_index = 0;
        while mmr_len >= index {
            // find the height of the tree by finding if we can subtract the  height +1
            height_counter += 1;
            actual_height_index = index;
            index = (1 << (height_counter + 2)) - 2;
        }
        (height_counter, actual_height_index)
    }

    fn find_bagging_indexes(&self, mut height: i64, index: usize, peaks: &mut Vec<H256>) {
        let mut new_index = index + (1 << (height + 1)) - 1; // go the potential right sibling
        while (new_index > self.get_last_added_index()) && (height > 0) {
            // lets go down left child till we hit a valid node or we reach height 0
            new_index -= 1 << height;
            height -= 1;
        }
        if (new_index <= self.get_last_added_index()) && (height >= 0) {
            // is this a valid peak which needs to be bagged
            peaks.push(self.mmr[new_index].hash);
            self.find_bagging_indexes(height, new_index, peaks); // lets go look for more peaks
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
        Some(self.mmr[index].hash)
    }

    /// This function returns the hash proof tree of a given hash.
    /// If the given hash is not in the tree, the vec will be empty.
    /// The Vec will be created in form of the Lchild-Rchild-parent(Lchild)-Rchild-parent-..
    /// This pattern will be repeated until the parent is the root of the MMR
    pub fn get_hash_proof<D: Digest>(&self, hash: &H256) -> Vec<H256> {
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

        if self.position.index == self.get_last_added_index() {
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
                result.push(peaks[i - 2]);
                result.push(peaks[i - 1]);
            } else if peaks[i - 1] == result[result.len() - 1] {
                result.insert(result.len() - 1, peaks[i - 2]);
                was_on_correct_height = true;
            } else if peaks[i - 2] == result[result.len() - 1] {
                result.push(peaks[i - 1]);
                was_on_correct_height = true;
            }

            let mut hasher = D::new();
            hasher.update(peaks[i - 2]);
            hasher.update(peaks[i - 1]);
            peaks[i - 2] = hasher.finalize().to_vec().into();
            i -= 1;
        }
        // lets calculate the final new peak
        let mut hasher = D::new();
        hasher.update(self.mmr[self.position.index].hash);
        hasher.update(peaks[0]);
        if was_on_correct_height {
            // edge case, our node is in the largest peak, we have already added it
            result.push(self.mmr[self.position.index].hash);
        }
        result.push(peaks[0]);
        result.push(hasher.finalize().to_vec().into());

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
            results.push(self.mmr[index].hash);
            return;
        }
        if sibling < index {
            results.push(self.mmr[sibling].hash);
            results.push(self.mmr[index].hash);
        } else {
            results.push(self.mmr[index].hash);
            results.push(self.mmr[sibling].hash);
            next_index = sibling + 1;
        }
        self.get_ordered_hash_proof(next_index, results);
    }
    /// This function returns the peak height of the mmr
    pub fn get_peak_height(&self) -> usize {
        self.position.height
    }
    /// This function will return the single merkle root of the MMR.
    pub fn get_merkle_root<D: Digest>(&self) -> H256 {
        let mut peaks = self.bag_mmr();
        let mut i = peaks.len();
        while i > 1 {
            // lets bag all the other peaks
            let mut hasher = D::new();
            hasher.update(peaks[i - 2]);
            hasher.update(peaks[i - 1]);
            peaks[i - 2] = hasher.finalize().to_vec().into();
            i -= 1;
        }
        if !peaks.is_empty() {
            // if there was other peaks, lets bag them with the highest peak
            let mut hasher = D::new();
            hasher.update(self.mmr[self.position.index].hash);
            hasher.update(peaks[0]);
            return hasher.finalize().to_vec().into();
        }
        // there was no other peaks, return the highest peak
        self.mmr[self.position.index].hash
    }
    // This function is just a private function to return the index of the last added node
    fn get_last_added_index(&self) -> usize {
        self.mmr.len() - 1
    }
    /// This function will verify the provided proof. Internally it uses the get_hash_proof function to construct a
    /// similar proof. This function will return true if the proof is valid
    /// If the order does not match Lchild-Rchild-parent(Lchild)-Rchild-parent-.. the validation will fail
    /// This function will only succeed if the given hash is of height 0
    pub fn verify_proof<D: Digest>(&self, hashes: &Vec<H256>) -> bool {
        if hashes.is_empty() {
            return false;
        }
        if self.get_object(&hashes[0]).is_none() && self.get_object(&hashes[1]).is_none() {
            // we only want to search for valid object's proofs, either 0 or 1 must be a valid object
            return false;
        }
        let proof = self.get_hash_proof::<D>(&hashes[0]);
        hashes.eq(&proof)
    }
}

impl<T> std::fmt::Display for MerkleMountainRange<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.mmr)
    }
}
