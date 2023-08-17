use std::fs;

use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    WalkDir::new("/Users/gnana.ganesh/workspace/Rust")
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| {
            if x.path().ends_with("target") {
                println!("{}", x.path().display());

                fs::remove_dir_all(x.path()).unwrap();
                println!("Directory was successfully deleted");
            }
        });
}
