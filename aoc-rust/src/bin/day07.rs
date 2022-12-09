use std::{
    cell::RefCell,
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
    rc::Rc,
};

#[derive(Debug)]
struct FileDescriptor {
    name: String,
    size: u64,
}

#[derive(Debug)]
struct FileEntry {
    size: u64,
}

impl FileEntry {
    fn new(size: u64) -> Self {
        Self { size }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DirectoryDescriptor {
    name: String,
}

#[derive(Debug)]
struct DirectoryEntry {
    cached_size: Option<u64>,
    child_files: HashMap<String, FileEntry>,
    child_directories: HashMap<String, Rc<RefCell<DirectoryEntry>>>,
}

impl DirectoryEntry {
    fn new() -> Self {
        Self {
            cached_size: None,
            child_files: HashMap::default(),
            child_directories: HashMap::default(),
        }
    }

    fn size(&mut self, use_cache: bool) -> u64 {
        if use_cache {
            if let Some(size) = self.cached_size {
                return size;
            }
        }
        let mut size = 0;
        for f in self.child_files.values() {
            size += f.size
        }
        for d in self.child_directories.values() {
            let mut d = d.borrow_mut();
            size += d.size(use_cache);
        }
        self.cached_size = Some(size);
        size
    }
}

struct DirectoryIter {
    next: Vec<Rc<RefCell<DirectoryEntry>>>,
}

impl From<&FileSystem> for DirectoryIter {
    fn from(fs: &FileSystem) -> Self {
        Self {
            next: vec![fs.root.clone()],
        }
    }
}

impl Iterator for DirectoryIter {
    type Item = Rc<RefCell<DirectoryEntry>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.pop()?;
        {
            let next = next.borrow();
            self.next
                .extend(next.child_directories.values().into_iter().cloned());
        }
        Some(next)
    }
}

#[derive(Debug)]
struct FileSystem {
    navigation: Vec<Rc<RefCell<DirectoryEntry>>>,
    root: Rc<RefCell<DirectoryEntry>>,
}

impl Default for FileSystem {
    fn default() -> Self {
        let root = Rc::new(RefCell::new(DirectoryEntry::new()));
        FileSystem {
            navigation: vec![root.clone()],
            root,
        }
    }
}

impl FileSystem {
    fn update(&mut self, command: Command) -> std::io::Result<()> {
        match command.program {
            Program::Cd(directory) => self.cd(directory),
            Program::Ls => {
                for output in command.outputs {
                    match output {
                        ProgramOutput::File(FileDescriptor { name, size }) => {
                            self.touch(name, *size)
                        }
                        ProgramOutput::Directory(DirectoryDescriptor { name }) => self.mkdir(name),
                    }
                }
                Ok(())
            }
        }
    }

    fn cd(&mut self, name: &str) -> std::io::Result<()> {
        match name {
            "/" => {
                self.navigation.clear();
                self.navigation.push(self.root.clone());
            }
            ".." => {
                self.navigation.pop();
            }
            _ => {
                let child_directory = self
                    .navigation
                    .last()
                    .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "corrupted"))?
                    .borrow()
                    .child_directories
                    .get(name)
                    .cloned();
                match child_directory {
                    None => {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, "corrupted"))
                    }
                    Some(directory) => self.navigation.push(directory.clone()),
                }
            }
        }
        Ok(())
    }

    fn mkdir(&mut self, name: &str) {
        if let Some(current_directory) = self.navigation.last_mut() {
            let mut current_directory = current_directory.borrow_mut();
            current_directory.child_directories.insert(
                String::from(name),
                Rc::new(RefCell::new(DirectoryEntry::new())),
            );
        }
    }

    fn touch(&mut self, name: &str, size: u64) {
        if let Some(current_directory) = self.navigation.last_mut() {
            let mut current_directory = current_directory.borrow_mut();
            current_directory
                .child_files
                .insert(String::from(name), FileEntry::new(size));
        }
    }

    fn size(&self, use_cache: bool) -> u64 {
        let mut root = self.root.borrow_mut();
        root.size(use_cache)
    }
}

#[derive(Debug)]
enum Program {
    Cd(String),
    Ls,
}

impl Program {
    fn parse<'a, T>(tokens: &mut Peekable<T>) -> Result<Self, std::io::Error>
    where
        T: Iterator<Item = &'a str>,
    {
        let leading_token = tokens.next().ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "expecting token '$'",
        ))?;
        assert_eq!("$", leading_token);

        match tokens.next() {
            Some("cd") => {
                tokens
                    .next()
                    .map(|p| Program::Cd(String::from(p)))
                    .ok_or(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "expecting 1 argument",
                    ))
            }
            Some("ls") => Ok(Program::Ls),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "program does not exist",
            )),
        }
    }
}

#[derive(Debug)]
enum ProgramOutput {
    File(FileDescriptor),
    Directory(DirectoryDescriptor),
}

impl ProgramOutput {
    fn parse<'a, T>(tokens: &mut Peekable<T>) -> std::io::Result<Self>
    where
        T: Iterator<Item = &'a str>,
    {
        match tokens.next() {
            Some("dir") => tokens
                .next()
                .map(|p| {
                    ProgramOutput::Directory(DirectoryDescriptor {
                        name: String::from(p),
                    })
                })
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "expecting directory's name",
                )),
            Some(s) => {
                let file_size = s
                    .parse::<u64>()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                tokens
                    .next()
                    .map(|p| {
                        ProgramOutput::File(FileDescriptor {
                            name: String::from(p),
                            size: file_size,
                        })
                    })
                    .ok_or(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "expecting directory's name",
                    ))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "unknown output",
            )),
        }
    }
}

struct Command<'a> {
    program: &'a Program,
    outputs: &'a [ProgramOutput],
}

impl<'a> Command<'a> {
    fn new(program: &'a Program, outputs: &'a [ProgramOutput]) -> Self {
        Self { program, outputs }
    }
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");

    let mut file_system = FileSystem::default();
    let mut current_program = None;
    let mut current_outputs = Vec::default();

    let reader = BufReader::new(File::open(&fpath).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace().peekable();
        while let Some(token) = tokens.peek() {
            match token {
                &"$" => {
                    if let Some(ref program) = current_program {
                        let command = Command::new(program, &current_outputs);
                        file_system.update(command).unwrap();
                    }
                    if let Ok(program) = Program::parse(&mut tokens) {
                        current_program.replace(program);
                        current_outputs.clear();
                    }
                }
                _ => {
                    if let Ok(output) = ProgramOutput::parse(&mut tokens) {
                        current_outputs.push(output);
                    }
                }
            }
        }
    }

    if let Some(ref program) = current_program {
        let command = Command::new(program, &current_outputs);
        file_system.update(command).unwrap();
    }

    let mut total = 0;
    for directory in DirectoryIter::from(&file_system) {
        let mut directory = directory.borrow_mut();
        let size = directory.size(true);
        if size <= 100000 {
            total += size;
        }
    }
    println!("{:?}", total);

    let total = file_system.size(true);
    let mut min_deleted_size = u64::MAX;
    for directory in DirectoryIter::from(&file_system) {
        let mut directory = directory.borrow_mut();
        let size = directory.size(true);
        if total - size <= 40000000 {
            min_deleted_size = min_deleted_size.min(size);
        };
    }
    println!("{:?}", min_deleted_size);
}
