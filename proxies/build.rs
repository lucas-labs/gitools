use std::{fs, io, path::Path};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    // copy the entire ./tldr directory to the ../target/{profile}/ directory
    let profile = std::env::var("PROFILE").unwrap();

    copy_dir_all("tldr", format!("../target/{}/tldr", profile)).unwrap();
}
