pub mod storage;
pub mod buffer;

use anyhow::Error;
use storage::file_manager::FileManager;
use storage::file_manager::page::Block;


pub const BLOCK_SIZE: usize = 4096; // page size in bytes; 
pub const DB_DIR: &str = "rustub_db"; // project directory; 

fn main() -> Result<(),Error> {
    let mut db_path = dirs::home_dir().unwrap();
    db_path.push(DB_DIR);
    db_path.push("testdb");
    FileManager::new(db_path,4)?;
    Ok(())
}
