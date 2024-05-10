use std::path::PathBuf;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
use std::fs;


pub struct FileWriter {
    file: fs::File,
    offset: u64,
}

impl FileWriter {
    pub fn new(file: fs::File) -> Self {
        Self {
            file,
            offset: 0
        }
    }
    
    pub fn get_offser(&self) -> u64 {
        self.offset
    }

    pub fn write_data(&mut self,  buf: &[u8]) -> Result<(), String> {
        match self.file.seek(SeekFrom::Start(self.offset)) {
            Ok(_) => (),
            Err(_) => return Err("Can't write archive file".to_string())
        }
        
        match self.file.write(buf) {
            Ok(_) => (),
            Err(_) => return Err("Can't write archive file".to_string())
        }
                
        self.offset += buf.len() as u64;
        
        Ok(())
    }

    fn save_table_position(&mut self) -> Result<(), String> {
        let offset = self.offset;
        self.offset = 0;
        self.write_data(&offset.to_ne_bytes())?;
        self.offset = offset;

        Ok(())
    }
}

struct FileTableEntry {
    path: PathBuf,
    size: u64,
    position: u64,
}

impl FileTableEntry {
    pub fn write_to_file(&self, writer: &mut FileWriter) -> Result<(), String> {
        let str_path = match self.path.to_str() {
            Some(path) => path,
            None => return Err("Can't write file table".to_string())
        };
        writer.write_data(&str_path.len().to_ne_bytes())?;
        writer.write_data(str_path.as_bytes())?;
        
        writer.write_data(&self.size.to_ne_bytes())?;
        writer.write_data(&self.position.to_ne_bytes())?;

        Ok(())
    }
}

pub struct FileTable {
    files: Vec<FileTableEntry>
}

impl FileTable {
    pub fn new() -> Self {
        Self {
            files: Vec::new()
        }
    }

    pub fn add_file(&mut self, path: PathBuf, size: u64, position: u64) {
        self.files.push( FileTableEntry {
            path,
            size,
            position
        });
    }

    pub fn write_to_file(&self, writer: &mut FileWriter) -> Result<(), String> {
        writer.save_table_position()?;
        writer.write_data(&self.files.len().to_ne_bytes())?;

        for file in &self.files {
            file.write_to_file(writer)?;
        }

        Ok(())
    }
}