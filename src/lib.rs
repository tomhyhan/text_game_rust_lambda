use std::{fs, error::Error};

pub fn get_file (file_name: &str) -> Result<String, Box<dyn Error>>{
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}