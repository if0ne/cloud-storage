use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug)]
pub enum InodeIndex {
    Directory {
        children: HashMap<String, Box<Inode>>,
    },
    LargeFile {
        blocks: Vec<(Uuid, Vec<&'static str>)>,
    },
    SmallFile {
        start_file: Option<(Uuid, Vec<&'static str>)>,
        commits: Vec<(Uuid, Vec<&'static str>)>,
    },
}

#[derive(Debug)]
pub struct Inode {
    alias: String,
    inode: InodeIndex,
}

impl Inode {
    pub fn add_directory(&mut self, alias: &str) -> &mut Inode {
        self.inode.add_directory(alias)
    }

    pub fn add_small_file_in_root(&mut self, alias: &str) {
        self.inode.add_small_file(alias.to_string())
    }

    pub fn add_large_file_in_root(&mut self, alias: &str) {
        self.inode.add_large_file(alias.to_string())
    }
}

impl InodeIndex {
    pub fn add_directory(&mut self, alias: &str) -> &mut Inode {
        match self {
            InodeIndex::Directory { children, .. } => {
                let child = children.entry(alias.to_string()).or_insert_with(|| {
                    Box::new(Inode {
                        alias: alias.to_string(),
                        inode: InodeIndex::Directory {
                            children: HashMap::new(),
                        },
                    })
                });

                child
            }
            InodeIndex::LargeFile { .. } => {
                todo!("ERROR HANDLING")
            }
            InodeIndex::SmallFile { .. } => {
                todo!("ERROR HANDLING")
            }
        }
    }

    pub fn add_large_file(&mut self, filename: String) {
        match self {
            InodeIndex::Directory { children, .. } => {
                children.insert(
                    filename.clone(),
                    Box::new(Inode {
                        alias: filename,
                        inode: InodeIndex::LargeFile { blocks: vec![] },
                    }),
                );
            }
            InodeIndex::LargeFile { .. } => {
                todo!("ERROR HANDLING")
            }
            InodeIndex::SmallFile { .. } => {
                todo!("ERROR HANDLING")
            }
        }
    }

    pub fn add_small_file(&mut self, filename: String) {
        match self {
            InodeIndex::Directory { children, .. } => {
                children.insert(
                    filename.clone(),
                    Box::new(Inode {
                        alias: filename,
                        inode: InodeIndex::SmallFile {
                            start_file: None,
                            commits: vec![],
                        },
                    }),
                );
            }
            InodeIndex::LargeFile { .. } => {
                todo!("ERROR HANDLING")
            }
            InodeIndex::SmallFile { .. } => {
                todo!("ERROR HANDLING")
            }
        }
    }
}

#[derive(Debug)]
pub struct Namespace {
    pub root: Inode,
}

impl Namespace {
    pub fn new() -> Self {
        Self {
            root: Inode {
                alias: "".to_string(),
                inode: InodeIndex::Directory {
                    children: HashMap::new(),
                },
            },
        }
    }

    pub fn mk_dir(&mut self, path: impl AsRef<Path>) -> &mut Inode {
        let path = path.as_ref();
        let mut current_dir = &mut self.root;
        for dir in path.iter() {
            let dir = dir.to_string_lossy().to_string();
            current_dir = current_dir.add_directory(&dir);
        }

        current_dir
    }

    pub fn create_small_file(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let path = path.to_str().unwrap().rsplit_once('/');
        if let Some((path, _)) = path {
            let dir = self.mk_dir(path);
            dir.inode.add_small_file(filename);
        } else {
            self.root.add_small_file_in_root(&filename);
        }
    }

    pub fn create_large_file(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let path = path.to_str().unwrap().rsplit_once('/');
        if let Some((path, _)) = path {
            let dir = self.mk_dir(path);
            dir.inode.add_large_file(filename);
        } else {
            self.root.add_large_file_in_root(&filename);
        }
    }
}
