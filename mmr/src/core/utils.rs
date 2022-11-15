/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/

/// This function takes in the index and calculates if the node is the right child node or not.
/// If the node is the tree root it will still give the answer as if it is a child of a node.
/// This function is an iterative function as we might have to subtract the largest left_most tree.
pub fn is_node_right(index: usize) -> bool {
    let mut height_counter = 0;
    while index >= ((1 << height_counter + 2) - 2) {
        // find the height of the tree by finding if we can subtract the  height +1
        height_counter += 1;
    }
    let height_index = (1 << height_counter + 1) - 2;
    if index == height_index {
        // If this is the first peak then subtracting the height of first peak will be 0
        return false;
    };
    if index == (height_index + ((1 << height_counter + 1) - 1)) {
        // we are looking if its the right sibling
        return true;
    };
    // if we are here means it was not a right node at height counter, we therefor search lower
    let new_index = index - height_index - 1;
    is_node_right(new_index)
}

/// This function takes in the index and calculates the height of the node
/// This function is an iterative function as we might have to subtract the largest left_most tree.
pub fn get_node_height(index: usize) -> usize {
    let mut height_counter = 0;
    while index >= ((1 << height_counter + 2) - 2) {
        // find the height of the tree by finding if we can subtract the  height +1
        height_counter += 1;
    }
    let height_index = (1 << height_counter + 1) - 2;
    if index == height_index {
        // If this is the first peak then subtracting the height of first peak will be 0
        return height_counter;
    };
    if index == (height_index + ((1 << height_counter + 1) - 1)) {
        // we are looking if its the right sibling
        return height_counter;
    };
    // if we are here means it was not a right node at height counter, we therefor search lower
    let new_index = index - height_index - 1;
    get_node_height(new_index)
}

/// This function takes in the index and calculates the index of the sibling.
pub fn sibling_index(index: usize) -> usize {
    let height = get_node_height(index);
    let index_count = (1 << height + 1) - 1;
    if is_node_right(index) {
        index - index_count
    } else {
        index + index_count
    }
}
