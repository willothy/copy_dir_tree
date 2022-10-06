use std::{fs, io, path::PathBuf};

fn walk_dir(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir)? {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                paths.push(entry.path());
                paths.extend(walk_dir(&entry.path())?);
            }
        }
    }
    Ok(paths)
}

pub fn copy_dir_tree(from_root: &PathBuf, to_root: &PathBuf) -> io::Result<()> {
    let paths = walk_dir(&from_root)?;
    for path in paths {
        let p: PathBuf = path
            .components()
            .skip(from_root.components().count())
            .collect();
        fs::create_dir_all(to_root.join(p))?;
    }
    Ok(())
}
