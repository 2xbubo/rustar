mod archiver;
mod file_helpers;

fn extract_file(_file_path: &str, _folder_path: &str) -> Result<(), String> {
    Ok(())
}

fn error_wrong_args() -> Result<(), String> {
    Err("Wrong arguments. Usage:\nArchive: -a folder_path archive_name.rtr\nExtract: -e archive_name.rtr folder_path".to_owned())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let result = match args.len() {
        4 => match args[1].as_str() {
                "-a" => archiver::archive_folder(&args[2], &args[3]),
                "-e" => extract_file(&args[2], &args[3]),
                _    => error_wrong_args()
            }
        _ => error_wrong_args()
    };

    match result {
        Err(msg) => println!("Error: {}", msg),
        Ok(_) => println!("Success!")
    }
}