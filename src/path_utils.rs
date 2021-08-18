use std::path::{Path, PathBuf};

pub fn prefix_groups<P: AsRef<Path> + Clone>(paths: &[P]) -> Vec<(String, Vec<P>)> {
    let mut prefix_groups = Vec::new();
    let mut current_prefix = first_filestem_word(&paths[0].as_ref()).to_string();
    let mut current_group: Vec<P> = vec![paths[0].clone()];
    for path in &paths[1..] {
        let next_prefix = first_filestem_word(path.as_ref());
        if next_prefix != current_prefix {
            prefix_groups.push((current_prefix, current_group.clone()));
            current_group.clear();
            current_prefix = next_prefix.to_string();
        } 
        current_group.push(path.clone());
    }
    prefix_groups.push((current_prefix, current_group.clone()));
    prefix_groups
}

pub fn unwrap_filestem(p: &Path) -> &str {
    p.file_stem().unwrap().to_str().unwrap()
}

fn first_filestem_word(p: &Path) -> &str {
    let filestem = unwrap_filestem(p);
    if let Some((val, _)) = filestem.split_once("_") {
        val
    } else {
        filestem
    }
}

pub fn extension_matches_in_dir(dir: &Path, extension: &str) -> Vec<PathBuf> {
    std::fs::read_dir(dir).expect("Error getting dir entries")
    .into_iter().filter_map(|read_dir| read_dir.ok())
    .filter_map(|dir_entry| {
        let file_path = dir_entry.path();
        match file_path.extension() {
            Some(ext) if ext == extension => {
                Some(file_path)
            },
            _ => {
                None
            },
        }
    }).collect()
}
