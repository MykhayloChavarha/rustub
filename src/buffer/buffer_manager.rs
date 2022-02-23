use std::{cmp::Ordering, collections::{HashMap, VecDeque}, iter::Inspect, rc::Rc, time::{Duration, Instant}};
use anyhow::{Error};
use priority_queue::PriorityQueue;

use crate::{BLOCK_SIZE};

type PageBuffer = [u8;BLOCK_SIZE];

mod replacer;

/// Module BufferManager. 
/// It is responsible for moving pages between disk and memory using LRU-1 replacement policy. 
pub struct BufferManager {
    // free_pages: VecDeque<Page>, // list of free pages.  
    // buffer_pool: VecDeque<Page>, // 
    // page_table: HashMap<PageId,Rc<Page>>, // owning pointer to 
    // replacer: Replacer,
    // clock: Instant
}

// impl BufferManager {
//     pub fn new(capacity: usize) -> BufferManager {
//         let mut free_pages: VecDeque<Page> = VecDeque::with_capacity(capacity);
//         for _ in 0..capacity {
//             free_pages.push_back(Page::new([0;BLOCK_SIZE]));
//         }
//         let buffer_pool = VecDeque::with_capacity(capacity);
//         let page_table = HashMap::with_capacity(capacity);
//         let replacer = Replacer::new(capacity);
//         let clock = Instant::now();


//         BufferManager {
//             free_pages,
//             buffer_pool,
//             page_table,
//             replacer,
//             clock
//         }
//     }

//     /// Create new page and pin it in buffer pool. 
//     pub fn new_page(&mut self, page_id: PageId) -> Result<(), Error> {
//         // check if there a free buffer and if yes create a new page and add it to the buffer pool 
//         // otherwise evict a page from buffer pool
//         Rc::from(page_id);
//         Ok(())
//     }

//     /// gets an existing page from the buffer pool 
//     pub fn get_page(&mut self) -> Result<(), Error> {
//         Ok(())
//     }
//     // pub fn delete_page(&mut self, page_id: PageId) -> Result

//     // pin specific page in the buffer pool and return reference to the page; 
//     // pub fn pin(&mut self, page_id: PageId) -> Option<Rc<Page>> {
//     //     if let Some(page) = self.buffer_pool.get_mut(page_id) {

//     //     }
//     //     None
//     // }

//     // // unpin the page and optionally mark it as dirty x
//     // pub fn unpin_page(page_id: PageId, set_dirty: bool) {

//     // }

//     // // flush page data to disk 
//     // pub fn flush_page(page_id: PageId) -> Result<(), Error> {
//     //     Ok(())
//     // }

//     // pub fn flush_all_pages() -> Result<(), Error> {
//     //     Ok(())
//     // }
// }

// #[derive(Hash,PartialEq, Eq)]
// struct Page {
//     buffer: [u8;BLOCK_SIZE],
//     pin_count: usize, 
//     is_dirty: bool
// }


// // each page can only own one buffer; 
// impl Page {
//     fn new(buffer: [u8;BLOCK_SIZE]) -> Page {
//         Page {
//             buffer,
//             pin_count: 0,
//             is_dirty: false,
//         }
//     }
//  }

// struct Replacer {
//     pages: PriorityQueue<Rc<Page>,Duration>
// }

// impl Replacer {
//     fn new(capacity:usize) -> Replacer {
//         let pages = PriorityQueue::with_capacity(capacity);
//         Replacer {
//             pages
//         }
//     }

//     fn insert(page_ptr: Rc<Page>) -> Result<(), Error> {
//         Ok(())
//     }

//     fn evict() -> Result<(), Error> {
//         Ok(())
//     }
// }



