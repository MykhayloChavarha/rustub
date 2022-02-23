use std::collections::HashMap; 
use std::fmt::Display;
use std::fs::{self, OpenOptions,File};
use std::io::{self, ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::str::FromStr; 
use dirs::home_dir;

use std::error::Error;

pub mod page; 

use crate::{BLOCK_SIZE};

use page::{Page, Block};

/// Module file manager. 
/// It is responsible for writing page buffers to disk. 
/// 
// we need to have a list of files 
pub struct FileManager {
    files: HashMap<String,File>,
    db_dir_path: PathBuf
}

/// File manager 
impl FileManager {
    /// Creates a new instance of a File Manager. 
    pub fn new(db_dir_path: PathBuf, capacity: usize) -> Result<FileManager, FileManagerError> {
        // let mut db_dir = env::current_dir()?;
        // let mut db_dir = home_dir();
        // db_dir.push(PARENT_DIR);
        // db_dir.push(db_name.to_string());

        // let db_dir_path = db_dir.as_path();
        if !db_dir_path.exists() {
            fs::create_dir(db_dir_path.clone())?;
        }

        let files = HashMap::with_capacity(capacity);
        Ok(FileManager {
            files,
            db_dir_path
        })
    }

    /// Create a new file that corresponds to a table. 
    /// Errors can occur during creation of a file, but also at the same time they can occur 
    /// if the file already exists or open. 
    /// 
    pub fn create(&mut self, file_name: &str) -> Result<(), FileManagerError> {
        let mut file_path = self.db_dir_path.clone();
        file_path.push(file_name);

        if file_path.exists() {
            return Err(FileManagerError::FileExists);
        }

        let new_file = File::create(file_path)?;
        drop(new_file);

        Ok(())
    }

    /// deletes table with corresponding table id or returns an error if file does not exist;  
    pub fn delete(&mut self, file_name: &str) -> Result<(), FileManagerError> {
        let mut file_path = self.db_dir_path.clone();
        file_path.push(file_name);

        if !file_path.exists() {
            return Err(FileManagerError::FileExists);
        }

        fs::remove_file(file_path)?;
        Ok(())
    }

    /// Opens a file with corresponding table id and adds it to the list of open files. 
    /// If corresponding file is already open returns an error.   
    /// 
    pub fn open(&mut self, file_name: &str) -> Result<(), FileManagerError> {
        let mut file_path = self.db_dir_path.clone();
        file_path.push(file_name);

        match self.files.contains_key(file_name) {
            true => Err(FileManagerError::FileAlreadyOpen),
            false => {
                let new_file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(file_path)?;
    
                self.files.insert(file_name.to_string(), new_file);
                Ok(())
            }
        }
    }

    /// close a file with corresponding table id. 
    pub fn close(&mut self, file_name: &str) -> Result<(), FileManagerError> {
        let mut file_path = self.db_dir_path.clone();
        file_path.push(file_name);

        match self.files.remove(file_name) {
            Some(file) => {drop(file); Ok(())},
            None => Err(FileManagerError::FileNotFound)
        }   
    }

    /// read the specific page into page buffer;
    pub fn read(&mut self, page: &mut Page, block: &Block) -> Result<(), FileManagerError> {
        match self.files.get_mut(&block.file_name) {
            Some(file) => {
                let offset = (BLOCK_SIZE as u64)*(block.block_number as u64);
                file.seek(SeekFrom::Start(offset))?;
                file.read(&mut page.page_buffer)?;
                Ok(())
            }
            None => Err(FileManagerError::FileNotFound),
        }  
    }

    /// write contents of the page buffer into a file; 
    pub fn write(&mut self, page: &mut Page, block: &Block) -> Result<(), FileManagerError> {
        match self.files.get_mut(&block.file_name) {
            Some(file) => {
                let offset = (BLOCK_SIZE as u64)*(block.block_number as u64);
                file.seek(SeekFrom::Start(offset))?;
                file.write(&mut page.page_buffer)?;
                file.sync_all()?;
                Ok(())
            }
            None => Err(FileManagerError::FileNotFound),
        }  
    }

    /// extends the hard drive space for new pages; 
    pub fn allocate(&mut self, file: &str, num_pages: usize) -> Result<(), FileManagerError> {
        // let maybe_file = self.iles.get_mut(table_id);

        // if let Some(file) = maybe_file {
        //     let metadata = file.metadata()?;
        //     let size = metadata.len();
        //     file.set_len(size+(BLOCK_SIZE as u64)*1000)?; // magic number
        //     file.sync_all()?;
        //     return Ok(());
        // }

        // Err(Error::from(NotFound))

        Ok(())
    }
}

#[derive(Debug)]
pub enum FileManagerError {
    IoErr(io::Error),
    FileExists, 
    FileNotFound,
    FileAlreadyOpen
    // error creating db directory 
    // error creating file
    // 
}

impl Display for FileManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileManagerError::IoErr(io_err)
                => write!(f,"IO error occured. Error kind: {:?}", io_err.kind()),
            FileManagerError::FileExists
                => write!(f, "File already exists"),
            FileManagerError::FileNotFound 
                => write!(f, "File not found"),
            FileManagerError::FileAlreadyOpen 
                => write!(f, "File already opne")
        }
    }
}

impl std::error::Error for FileManagerError {}

impl From<io::Error> for FileManagerError {
    fn from(err: io::Error) -> Self {
        FileManagerError::IoErr(err)
    }
}


#[cfg(test)]
mod tests {
    use super::*; 
    use fs::remove_dir;

    // #[test]
    // fn create_db_dir() -> Result<(),Error> {
    //     let db_name = "test_db";
    //     let capacity = 2; 
    //     let mut db_dir = env::current_dir()?;
    //     db_dir.push(PARENT_DIR);
    //     db_dir.push(db_name);
    //     FileManager::new(db_name,capacity)?;
    //     let db_path = db_dir.as_path();
    //     assert_eq!(db_path.exists(),true);
    //     remove_dir(db_path)?;
    //     assert_ne!(db_path.exists(),true);
    //     Ok(())
    // }
    // fn create_new_file() -> Result<(), Error> {
    //     let db_path = env::current_dir()?;
    //     db_path.push(PARENT_DIR);
    //     db_path.push("test_db");
    //     db_path.push("table1");
    //     let fm = FileManager::new("test_db",2)?;
    //     // let page_id = PageId::new("table1".to_string(),5);
    //     fm.create("table1")?;
    //     let metadata=File::metadata()
    //     // assert that file exists
         
    // }
}
