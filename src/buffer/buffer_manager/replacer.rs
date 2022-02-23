use std::{time::{Duration}};

use priority_queue::DoublePriorityQueue;

use crate::BLOCK_SIZE;

/// struct replacer, it is responsible for moving 
pub(super) struct Replacer {
    container: DoublePriorityQueue<Page,Duration>
}

impl Replacer {
    pub(super) fn new(capacity: usize) -> Replacer {
        Replacer {  
            container: DoublePriorityQueue::with_capacity(capacity)
        }
    }
    /// Evict victim page or return None if replacer is empty. 
    pub(super) fn evict(&mut self) -> Option<Page> {
        if let Some(victim) = self.container.pop_min() {
            return Some(victim.0);
        }
        None
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct PageId {
    table_id: String, 
    page_id: u64,
}

impl PageId {
    pub fn new(table_id:&str, page_id: u64) -> PageId {
        PageId {
            table_id: table_id.to_string(),
            page_id: page_id
        }
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Page {
    is_dirty: bool, // marks the page in the frame as dirty
    pin_count: u32, // number of times the page was pinned in the buffer pool 
    buffer: [u8;BLOCK_SIZE],
    access_time: Duration
}


impl Page {
    pub fn new(buffer: [u8;BLOCK_SIZE], access_time:Duration) -> Page {
        Page {
            is_dirty: false, 
            pin_count: 0,
            buffer,
            access_time
        }
    }

    pub(super) fn reset(&mut self) {
        self.is_dirty = false; 
        self.pin_count = 0;
        self.buffer.fill(Default::default());
        self.access_time = Duration::ZERO;
    }



    pub(super) fn pin(&mut self, access_time:Duration) {
        self.pin_count=self.pin_count+1; 
        self.access_time = access_time
    }

    fn unpin(&mut self) {
        if self.pin_count > 0 {
            self.pin_count = self.pin_count + 1; 
        }
    }

    pub(super) fn update_access_time(&mut self, access_time: Duration) {
        self.access_time = access_time;
    }
}
