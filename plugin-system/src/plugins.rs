extern crate libloading as lib;

use std::{
    ffi::OsStr,
    fs::{DirEntry, read_dir},
    path::PathBuf,
};

#[cfg(unix)]
use lib::os::unix::Library;
#[cfg(windows)]
use lib::os::windows::Library;

use crate::read_dirs;

#[cfg(unix)]
const DYNAMIC_LIBRARY_EXTENSION: &str = ".so";

#[cfg(windows)]
const DYNAMIC_LIBRARY_EXTENSION: &str = ".dll";

type IOResult<T> = std::io::Result<T>;

pub struct Libraries {
    dirs: Box<dyn Iterator<Item=IOResult<DirEntry>>>,
}

pub struct PlugIn {
    pub name: String,
    pub path: PathBuf,
    pub library: Library,
}

impl Iterator for Libraries {
    type Item = Result<PlugIn, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dirs.next() {
            Some(Ok(entry)) => {
                let path = entry.path();
                let library = Library::new(path.file_name().unwrap_or_else(|| OsStr::new("")));

                match library {
                    Ok(library) => {
                        let plug_in = PlugIn {
                            name: get_plugin_name(&path),
                            path: entry.path(),
                            library,
                        };

                        Some(Ok(plug_in))
                    }
                    Err(err) => Some(Err(Box::new(err)))
                }
            }
            Some(Err(err)) => Some(Err(Box::new(err))),
            None => None
        }
    }
}

pub fn scan_libraries(root: &str) -> IOResult<Libraries> {
    Ok(Libraries {
        dirs: Box::new(read_dir(root)?.filter(is_dynamic_library)),
    })
}

pub fn scan_all_libraries(root: &str) -> IOResult<Libraries> {
    Ok(Libraries {
        dirs: Box::new(read_dirs(root)?.filter(is_dynamic_library)),
    })
}

#[cfg(windows)]
#[inline]
fn get_plugin_name(path: &PathBuf) -> String {
    String::from(path.file_stem().unwrap_or_else(|| OsStr::new("")).to_str().unwrap_or(""))
}

#[cfg(unix)]
#[inline]
fn get_plugin_name(path: &PathBuf) -> String {
    // truncates "lib" prefix for rust unix dynamic libraries, i.e. "lib*.so"
    String::from(&path.file_stem().unwrap_or_else(|| OsStr::new("")).to_str().unwrap_or("")[3..])
}

fn is_dynamic_library(entry: &IOResult<DirEntry>) -> bool {
    match entry {
        Ok(entry) => {
            let path = entry.path();

            path.is_file() && path.exists() &&
                path.as_os_str().to_str().unwrap_or("").ends_with(DYNAMIC_LIBRARY_EXTENSION)
        }
        Err(_) => false
    }
}
