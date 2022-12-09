use std::collections::HashMap;

use tracing::debug;

type SomePath = String;

#[derive(Clone, Debug)]
pub enum Stream {
    Input(Command),
    Output(File),
}

#[derive(Clone, Debug)]
pub enum Command {
    Change(SomePath),
    ChangeRelativeUp,
    List,
}

#[derive(Clone, Debug)]
pub struct PathData {
    pub size: usize,
    pub path: String,
}

impl Default for PathData {
    fn default() -> Self {
        Self { size: 0, path: "".to_string() }
    }
}

impl PathData {
    pub fn new(path: &str, size: usize) -> Self {
        Self { size, path: path.to_string() }
    }
    pub fn with_path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }
    pub fn build(&mut self) -> Self {
        Self {
            size: self.size,
            path: self.path.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum File {
    Directory(PathData),
    File(PathData),
}

impl File {
    pub fn size(&self) -> usize {
        match self {
            File::Directory(data) => data.size,
            File::File(data) => data.size,
        }
    }
}

pub fn find_good_deletion_candidates(log: &str) -> usize {
    folder_sizes_below(log, 100_000)
        .iter()
        .map(|(_path, file)| file.size())
        .sum()
}

pub fn folder_to_delete(log: &str) -> usize {
    let used_space = largest_folder_size(log);
    let total_space_available = 70_000_000;
    let space_needed = 30_000_000;
    let space_to_delete = used_space - (total_space_available - space_needed);
    let deleted = folder_sizes_above(log, space_to_delete);
    deleted
}

pub fn largest_folder_size(log: &str) -> usize {
    build_sizes(log)
        .values()
        .map(|v| v.size())
        .max()
        .unwrap()
}

pub fn folder_sizes_above(log: &str, upper_threshold: usize) -> usize {
    // let sizes = build_sizes(log)
    //     .iter()
    //     .filter(|(_file_path, file)| matches!(file, File::Directory(file_data) if file_data.size <= 100_000))
    //     .map(|(fp, fd)| (fp.clone(), fd.clone()))
    //     .collect::<HashMap<_, _>>()
    //     ;
    build_sizes(log)
        .iter()
        .filter(|(_file_path, file)| {
            debug!("{_file_path}: {}", file.size());
            matches!(file, File::Directory(file_data) if file_data.size >= upper_threshold)
        })
        .map(|(_file_path, file_data)| file_data.size())
        .min()
        .unwrap()
}

pub fn folder_sizes_below(log: &str, upper_threshold: usize) -> HashMap<String, File> {
    build_sizes(log)
        .iter()
        .inspect(|(fp, f)| debug!("Unfiltered - {fp}: {f:?}"))
        .filter(|(_file_path, file)| matches!(file, File::Directory(file_data) if file_data.size <= upper_threshold))
        .inspect(|(fp, f)| debug!("  Filtered - {fp}: {f:?}"))
        .map(|(fp, fd)| (fp.clone(), fd.clone()))
        .collect()
}

pub fn build_sizes(log: &str) -> HashMap<SomePath, File> {
    let mut cwd: Option<String> = None;
    let mut sizes = HashMap::new();
    for line in log.split('\n').filter(|l| !l.trim().is_empty()) {
        match tokenize(line) {
            Stream::Input(Command::Change(path)) => {
                match &cwd {
                    Some(s) => match s.as_str() {
                        "/" => cwd = Some(format!("/{path}")),
                        _ => cwd = Some(format!("{s}/{path}")),
                    }
                    None => {
                        cwd = Some(path.clone());
                    }
                };
            }
            Stream::Input(Command::ChangeRelativeUp) => {
                cwd = Some(dir(&cwd.clone().unwrap()).to_string());
            }
            Stream::Input(Command::List) => {
                // turns out this can just be ignored because no state is changed
            }
            Stream::Output(File::Directory(data)) => {
                let full_path = match &cwd {
                    Some(s) => match s.as_str() {
                        "/" => format!("/{}", &data.path),
                        _ => format!("{s}/{}", &data.path),
                    }
                    None => data.path.clone(),
                };
                sizes
                    .entry(full_path)
                    .or_insert(File::Directory(data));
            }
            Stream::Output(File::File(data)) => {
                let full_path = match &cwd {
                    Some(s) => match s.as_str() {
                        "/" => format!("/{}", &data.path),
                        _ => format!("{s}/{}", &data.path),
                    }
                    None => data.path.clone(),
                };
                let file_size = data.size;
                if !sizes.contains_key(&full_path) {
                    sizes.insert(full_path.clone(), File::File(data));
                }
                let mut parent_path = full_path.clone();
                loop {
                    if parent_path.is_empty() {
                        break;
                    }
                    match sizes.get_mut(&parent_path) {
                        Some(path_data) => {
                            if let File::Directory(p) = path_data {
                                debug!("  +  | {:>6}: +{} [now: {}] (fp={full_path})", parent_path, file_size, p.size + file_size);
                                p.size += file_size;
                            }
                        }
                        None => {
                            let parent_data = PathData::new(&parent_path, file_size);
                            debug!(" +++ | {:>6}: +{} [now: {}] (fp={full_path})", parent_path, file_size, file_size);
                            sizes.insert(parent_path.to_string(), File::Directory(parent_data));

                        }
                    }
                    parent_path = dir(&parent_path).to_string();
                }
            }
        }
    }
    sizes
}

pub fn dir(path: &str) -> &str {
    let count = path.chars().count();
    let folder_count = path.chars().filter(|c| *c == '/').count();
    if count == 0 {
        path
    } else if folder_count == 1 && count > 1 {
        &path[..1]
    } else {
        let pos = count
            - path
                .chars()
                .rev()
                .position(|c| c == '/')
                .unwrap_or_default()
            - 1;
        &path[..pos]
    }
}

pub fn tokenize(line: &str) -> Stream {
    if let Some(line) = line.strip_prefix("$ cd ") {
        if line == ".." {
            Stream::Input(Command::ChangeRelativeUp)
        } else {
            Stream::Input(Command::Change(line.to_string()))
        }
    } else if let Some(_line) = line.strip_prefix("$ ls") {
        Stream::Input(Command::List)
    } else if let Some(line) = line.strip_prefix("dir ") {
        Stream::Output(File::Directory(PathData::default().with_path(line).build()))
    } else {
        let (size_string, filename) = line
            .split_once(' ')
            .expect("Could not parse file line");
        let size = size_string
            .parse::<usize>()
            .expect("Could not parse file size");
        Stream::Output(File::File(PathData::new(filename, size)))
    }
}
