use core::convert::TryInto;

use crate::pager;
use crate::row;

// Node metadata values and offsets.
pub const NODE_TYPE_SIZE: usize = std::mem::size_of::<u8>();
pub const NODE_TYPE_OFFSET: usize = 0;
pub const IS_ROOT_SIZE: usize = std::mem::size_of::<u8>();
pub const IS_ROOT_OFFSET: usize = NODE_TYPE_OFFSET + NODE_TYPE_SIZE;
pub const PARENT_POINTER_SIZE: usize = std::mem::size_of::<u32>();
pub const PARENT_POINTER_OFFSET: usize = IS_ROOT_OFFSET + IS_ROOT_SIZE;
pub const COMMON_NODE_HEADER_SIZE: usize = NODE_TYPE_SIZE + IS_ROOT_SIZE + PARENT_POINTER_SIZE;

// Leaf nodes also have a numCells in their metadata block.
pub const LEAF_NODE_NUM_CELLS_SIZE: usize = std::mem::size_of::<u32>();
pub const LEAF_NODE_NUM_CELLS_OFFSET: usize = COMMON_NODE_HEADER_SIZE;
pub const LEAF_NODE_HEADER_SIZE: usize = COMMON_NODE_HEADER_SIZE + LEAF_NODE_NUM_CELLS_SIZE;

// Constants for the remaining data block.
pub const LEAF_NODE_KEY_SIZE: usize = std::mem::size_of::<u32>();
pub const LEAF_NODE_KEY_OFFSET: usize = 0;
pub const LEAF_NODE_VALUE_SIZE: usize = row::ROW_SIZE;
pub const LEAF_NODE_VALUE_OFFSET: usize = LEAF_NODE_KEY_OFFSET + LEAF_NODE_KEY_SIZE;
pub const LEAF_NODE_CELL_SIZE: usize = LEAF_NODE_KEY_SIZE + LEAF_NODE_VALUE_SIZE;
pub const LEAF_NODE_SPACE_FOR_CELLS: usize = pager::PAGE_SIZE - LEAF_NODE_HEADER_SIZE;
pub const LEAF_NODE_MAX_CELLS: usize = LEAF_NODE_SPACE_FOR_CELLS / LEAF_NODE_CELL_SIZE;

pub enum NodeType {
    Internal,
    Leaf,
}

pub struct LeafNode {
    data: Vec<u8>,
}

impl LeafNode {
    pub fn new() -> LeafNode {
        LeafNode {
            data: vec![0; pager::PAGE_SIZE],
        }
    }

    pub fn get_num_cells(&self) -> u32 {
        let value_region = &self.data
            [LEAF_NODE_NUM_CELLS_OFFSET..LEAF_NODE_NUM_CELLS_OFFSET + LEAF_NODE_NUM_CELLS_SIZE];
        return u32::from_ne_bytes(value_region.try_into().unwrap());
    }

    pub fn set_num_cells(&mut self, num_cells: u32) {
        let value_region = self.data
            [LEAF_NODE_NUM_CELLS_OFFSET..LEAF_NODE_NUM_CELLS_OFFSET + LEAF_NODE_NUM_CELLS_SIZE]
            .copy_from_slice(&num_cells.to_ne_bytes());
    }

    pub fn leaf_node_cell(&mut self, cell_num: u32) -> &mut [u8] {
        let cell_offset = LEAF_NODE_HEADER_SIZE + (cell_num as usize * LEAF_NODE_CELL_SIZE);
        return &mut self.data[cell_offset..cell_offset + LEAF_NODE_CELL_SIZE];
    }

    pub fn leaf_node_key(&mut self, cell_num: u32) -> &mut [u8] {
        return &mut self.leaf_node_cell(cell_num)[0..LEAF_NODE_KEY_SIZE];
    }

    pub fn leaf_node_value(&mut self, cell_num: u32) -> &mut [u8] {
        return &mut self.leaf_node_cell(cell_num)[LEAF_NODE_KEY_SIZE..];
    }
}
