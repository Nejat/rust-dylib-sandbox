use std::{
    ffi::OsStr,
    fs::DirEntry,
    io::ErrorKind,
};

use crate::read_dirs;

#[cfg(windows)]
const ROOT: &str = "..\\target";
#[cfg(windows)]
const SPLITTER: &str = "\\";

#[cfg(unix)]
const ROOT: &str = "../target";
#[cfg(unix)]
const SPLITTER: &str = "/";

type IOResult<T> = std::io::Result<T>;

#[test]
fn given_a_directory_of_files_read_dirs_should_traverse_all_child_directories() {
    match read_dirs(ROOT) {
        Ok(entries) => {
            let entries: Option<usize> = entries
                .into_iter()
                .map(
                    |e| e.unwrap()
                        .path().to_str().unwrap()
                        .split(SPLITTER)
                        .count()
                )
                .max();

            match entries {
                Some(entries) => assert!(entries > 2, format!("actual depth: {}", entries)),
                None => panic!("expected some child directory entries"),
            }
        }
        Err(err) => {
            panic!(format!("root: \"{}\", err: {:?}", ROOT, err));
        }
    }
}

#[test]
fn given_a_directory_of_files_read_dirs_should_traverse_all_child_directories_and_filter_entries() {
    #[cfg(windows)]
    const EXTENSION: &str = "dll";
    #[cfg(unix)]
    const EXTENSION: &str = "so";

    let mut count = 0;

    match read_dirs(ROOT) {
        Ok(entries) => {
            for entry in entries.filter(files_filtered) {
                match entry {
                    Ok(entry) => {
                        assert_eq!(entry.path().extension().unwrap_or_else(|| OsStr::new("")), OsStr::new(EXTENSION));
                        count += 1;
                    }
                    Err(err) => panic!(format!("root: \"{}\", err: {:?}", ROOT, err)),
                }
            }
        }
        Err(err) => panic!(format!("root: \"{}\", err: {:?}", ROOT, err)),
    }

    assert!(count > 1, format!("actual finds: {}", count));

    fn files_filtered(entry: &IOResult<DirEntry>) -> bool {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                path.is_file() && path.exists() &&
                    path.as_os_str().to_str().unwrap_or("").ends_with(EXTENSION)
            }
            Err(_) => false
        }
    }
}

#[test]
fn given_a_directory_that_does_not_exist_read_dirs_should_return_an_error() {
    match read_dirs("non-existing-target") {
        Ok(_) => panic!("not expecting any entries"),
        Err(err) => assert_eq!(err.kind(), ErrorKind::NotFound),
    }
}
