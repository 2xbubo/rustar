use std::path::Path;
use std::path::PathBuf;
use std::fs;
use crate::file_helpers;

fn get_files_in_folder(folder_path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let paths= match fs::read_dir(folder_path) {
        Ok(paths) => paths,
        Err(_) => return Err(format!("Can't read path {}", folder_path.display().to_string()))
    };    

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            files.push(path);
        }
        else {
            get_files_in_folder(&path, files)?;
        }
    }   

    Ok(())
}

fn read_file(file_path: &Path) -> Result<Vec<u8>, String> {
    match fs::read(file_path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("Can't read file {}", file_path.display().to_string()))
    }
}

pub fn archive_folder(folder_path: &str, file_path: &str) -> Result<(), String> {
    let file = match fs::File::create(file_path) {
        Ok(file) => file,
        Err(_) => return Err("Can't create archive file".to_string())
    };

    let mut files_paths: Vec<PathBuf> = Vec::new();
    get_files_in_folder(Path::new(folder_path), &mut files_paths)?;
    
    let mut file_table = file_helpers::FileTable::new();
    let mut writer = file_helpers::FileWriter::new(file);

    writer.write_data(&0u64.to_ne_bytes())?;

    for file_path in files_paths {
        let file_data = read_file(&file_path)?;

        file_table.add_file(file_path, file_data.len() as u64, writer.get_offser());

        writer.write_data(&file_data)?;
    }

    file_table.write_to_file(&mut writer)?;

    Ok(())
}