use std::{
    collections::VecDeque,
    fs::{DirEntry, read_dir, ReadDir},
    path::{
        Path,
        PathBuf,
    },
};

type IOResult<T> = std::io::Result<T>;

pub struct ReadDirs {
    current: Option<ReadDir>,
    dirs: VecDeque<PathBuf>,
}

pub fn read_dirs(root: &str) -> IOResult<ReadDirs> {
    Ok(
        ReadDirs {
            current: Some(read_dir(Path::new(root).to_path_buf())?),
            dirs: VecDeque::new(),
        }
    )
}

impl Iterator for ReadDirs {
    type Item = IOResult<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = &mut self.current {
                let next = current.next();

                match next {
                    Some(Ok(next)) => {
                        let path = next.path();

                        if path.is_dir() {
                            self.dirs.push_back(next.path());
                        }

                        if path.is_file() {
                            return Some(Ok(next));
                        }
                        continue;
                    }
                    Some(Err(err)) => {
                        return Some(Err(err));
                    }
                    None => {
                        let path = self.dirs.pop_front();

                        if let Some(path) = path {
                            let read_dir = read_dir(path);
                            let read_dir = match read_dir {
                                Ok(read_dir) => read_dir,
                                Err(err) => return Some(Err(err)),
                            };

                            self.current = Some(read_dir);

                            continue;
                        }
                    }
                }
            }

            break;
        }

        None
    }
}
