#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;
use std::path::Path;


#[napi]
fn delete_folder_recursive(item_path: String) -> napi::Result<()> {
    fs::remove_dir_all(item_path)?;

    Ok(())
}
#[napi]
fn copy_folder_recursive(source_path: String, target_path: String) -> napi::Result<()> {
    let source = Path::new(&source_path);
    let target = Path::new(&target_path);
    if !source.exists() {
        return Ok(());
    }

    let target_path = std::path::Path::new(target).join(source.file_name().unwrap());
    fs::create_dir_all(&target_path)?;

    let entries = fs::read_dir(source)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            copy_folder_recursive(entry_path.to_str().unwrap().to_string(), target_path.to_str().unwrap().to_string())?;
        } else {
            let target_file_path = target_path.join(entry.file_name());
            fs::copy(&entry_path, &target_file_path)?;
        }
    }

    Ok(())
}