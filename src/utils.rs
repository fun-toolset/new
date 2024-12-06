use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

use anyhow::{Context, Result};

pub fn create_directories(dirs: Vec<PathBuf>) -> Result<()> {
    for dir in dirs {
        create_dir_all(&dir).context(format!("Failure to create directory: {:?}", dir))?;
    }

    Ok(())
}

pub fn create_files(files: Vec<PathBuf>) -> Result<()> {
    for file in files {
        let path = file.clone();
        File::create(&path).context(format!("Failure to create file: {:?}", file))?;
    }

    Ok(())
}
