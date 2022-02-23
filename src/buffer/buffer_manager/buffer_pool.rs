use std::collections::VecDeque;
use anyhow::Error;

use crate::BLOCK_SIZE;

type Buffer = [u8; BLOCK_SIZE];
/// Modue Buffer pool, 
/// It operates on pinned pages; 
pub(super) struct BufferPool {
}

impl BufferPool {
    pub(super) fn new(buffer_pool_size: usize) -> BufferPool {
        BufferPool { 
        }
    } 
    
}
