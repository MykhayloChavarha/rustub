use std::collections::HashMap; 
use std::fmt::Display;
use std::fs::{self, OpenOptions,File};
use std::io::{self, ErrorKind, Read, Seek, SeekFrom, Write};
use std::os::unix::prelude::FileExt;
use std::path::PathBuf;
use std::str::FromStr; 
use dirs::home_dir;

use std::error::Error;

pub mod page; 

use crate::{BLOCK_SIZE, EXTENT_SIZE};

use page::{Page, Block};

/// Module file manager. 
/// It is responsible for writing page buffers to disk. 
/// 
///
pub struct FileManager {
    files: HashMap<String,File>,
    db_dir_path: PathBuf
}

/// File manager 
impl FileManager {
    /// Creates a new instance of a File Manager. 

    pub fn new(db_dir_path: PathBuf, capacity: usize) -> Result<FileManager, FileManagerError> {

        let path_exist = db_dir_path.exists();
        if !path_exist {
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
        new_file.sync_all()?;
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
            Some(file) => {
                drop(file); 
                Ok(())
            },
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
                //todo: validate block; 
                file.seek(SeekFrom::Start(offset))?;
                file.write(&mut page.page_buffer)?;
                file.sync_all()?;
                Ok(())
            }
            None => Err(FileManagerError::FileNotFound),
        }  
    }

    /// extends the hard drive space for new pages; 
    pub fn allocate(&mut self, file_name: &str) -> Result<(), FileManagerError> {
        // let maybe_file = self.iles.get_mut(table_id);

        match self.files.get_mut(file_name) {
            Some(file) => {
                let end = file.seek(SeekFrom::End(0))?;
                let chunk = (EXTENT_SIZE as u64)*(BLOCK_SIZE as u64);
                file.set_len(end + chunk)?;
                file.sync_all()?;
                Ok(())
            },
            None => Err(FileManagerError::FileNotFound),
        }
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
    use crate::DB_DIR;

    use super::*; 
    use fs::remove_dir;
    use anyhow::Error;

    #[test]
    fn should_create_db_folder_if_none_exist() -> Result<(), Error> {
        let mut db_path = home_dir().unwrap();
        db_path.push(DB_DIR);
        db_path.push("test_db");
        assert_eq!(db_path.exists(),false);
        FileManager::new(db_path.clone(),10)?;
        assert_eq!(db_path.exists(),true);
        fs::remove_dir(db_path)?;
        Ok(())
    }

    #[test]
    fn should_create_db_folder_if_one_exists() -> Result<(), Error> {
        let mut db_path = home_dir().unwrap();
        db_path.push(DB_DIR);
        db_path.push("test_db");
        fs::create_dir(db_path.clone())?;
        assert_eq!(db_path.exists(),true);
        FileManager::new(db_path.clone(),10)?;
        assert_eq!(db_path.exists(),true);
        fs::remove_dir(db_path)?;
        Ok(())
    }

    #[test]
    fn should_create_and_delete_files() -> Result<(), Error> {
        let mut db_path = home_dir().unwrap();
        db_path.push(DB_DIR);
        db_path.push("test_db");
        let mut fm = FileManager::new(db_path.clone(),10)?;
        fm.create("file1")?;
        fm.create("file2")?;
        let mut path_with_file1 = db_path.clone();
        path_with_file1.push("file1");
        let mut path_with_file2 = db_path.clone();
        path_with_file2.push("file2");
        // check if both file exist. 
        assert_eq!(path_with_file1.exists(),true);
        assert_eq!(path_with_file2.exists(),true);
        // delete files from the database 
        fm.delete("file1")?;
        fm.delete("file2")?;
        assert_eq!(path_with_file1.exists(),false);
        assert_eq!(path_with_file2.exists(),false);
        fs::remove_dir(db_path)?;
        Ok(())
    }

    // #[test]
    // fn should_create_db_folder_if_none_exist_v2() {
    //     println!("Running Test #2");
    //     let db_path = get_new_db_path("test_db");
    //     debug_assert_eq!(db_path.exists(),false);
    //     let mut fm = FileManager::new(db_path.clone(),10);
    //     match fm {
    //         Ok(fm) => {
    //             match fs::remove_dir(db_path.clone()) {
    //                 Ok(_) => assert!(true),
    //                 Err(_) => {println!("Error occured during dir removal");assert!(false)}
    //             }
    //         },
    //         Err(e) => {println!("Error occured during file_manager {}", e);assert!(false);},
    //     }
    // }

    // #[test]
    // fn should_create_db_folder_if_none_exist_ver2() {
    //     println!("Running Test #2");
    //     let db_path = get_new_db_path("test_db");
    //     debug_assert_eq!(db_path.clone().exists(),false);
    //     let mut fm = FileManager::new(db_path.clone(),10);
    //     match fm {
    //         Ok(fm) => {
    //             match fs::remove_dir(db_path.clone()) {
    //                 Ok(_) => assert!(true),
    //                 Err(_) => {println!("Error occured during dir removal");assert!(false)}
    //             }
    //         },
    //         Err(_) => {println!("Error occured during file_manager");assert!(false);},
    //     }
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
