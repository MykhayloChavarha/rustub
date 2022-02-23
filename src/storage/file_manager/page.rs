use crate::BLOCK_SIZE;

#[derive(Hash,PartialEq, Eq)]
pub struct Block {
    pub file_name: String, 
    pub block_number: usize
}

impl Block {
    pub fn new(table: String, block_number: usize) -> Block {
        Block { 
            file_name: table, 
            block_number
        }
    }
}

pub struct Page {
    pub(super) page_buffer: [u8;BLOCK_SIZE]
}

impl Page {
    pub fn new(page_buffer:[u8;BLOCK_SIZE]) -> Page {
        Page {
            page_buffer
        }
    }
    
}
