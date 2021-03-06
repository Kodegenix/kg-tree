use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::rc::{Rc, Weak};
use std::cell::Cell;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileFormat {
    Binary,
    Text,
    Json,
    Yaml,
    Toml,
}

impl FileFormat {
    pub fn from(f: &str) -> FileFormat {
        if f.eq_ignore_ascii_case("text") || f.eq_ignore_ascii_case("txt") {
            FileFormat::Text
        } else if f.eq_ignore_ascii_case("json") {
            FileFormat::Json
        } else if f.eq_ignore_ascii_case("yaml") || f.eq_ignore_ascii_case("yml") {
            FileFormat::Yaml
        } else if f.eq_ignore_ascii_case("toml") {
            FileFormat::Toml
        } else {
            FileFormat::Binary
        }
    }
}

impl<'a> std::convert::From<&'a str> for FileFormat {
    fn from(s: &'a str) -> Self {
        FileFormat::from(s)
    }
}

impl std::str::FromStr for FileFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FileFormat::from(s))
    }
}

impl std::fmt::Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FileFormat::Binary => write!(f, "binary"),
            FileFormat::Text => write!(f, "text"),
            FileFormat::Json => write!(f, "json"),
            FileFormat::Yaml => write!(f, "yaml"),
            FileFormat::Toml => write!(f, "toml"),
        }
    }
}

impl Default for FileFormat {
    fn default() -> Self {
        FileFormat::Binary
    }
}


#[derive(Debug)]
struct FileInfoInner {
    file_path: PathBuf,
    file_type: FileType,
    file_format: Cell<FileFormat>,
}

#[derive(Clone)]
pub struct FileInfo(Rc<FileInfoInner>);

impl FileInfo {
    pub fn new<P: Into<PathBuf> + AsRef<Path>>(
        file_path: P,
        file_type: FileType,
        file_format: FileFormat,
    ) -> FileInfo {
        debug_assert!(file_path.as_ref().is_absolute());

        FileInfo(Rc::new(FileInfoInner {
            file_path: file_path.into(),
            file_type,
            file_format: Cell::new(file_format),
        }))
    }

    pub fn new_file<P: Into<PathBuf> + AsRef<Path>>(
        file_path: P,
        file_format: FileFormat,
    ) -> FileInfo {
        debug_assert!(file_path.as_ref().is_absolute());

        FileInfo(Rc::new(FileInfoInner {
            file_path: file_path.into(),
            file_type: FileType::File,
            file_format: Cell::new(file_format),
        }))
    }

    pub fn new_dir<P: Into<PathBuf> + AsRef<Path>>(
        file_path: P,
    ) -> FileInfo {
        debug_assert!(file_path.as_ref().is_absolute());

        FileInfo(Rc::new(FileInfoInner {
            file_path: file_path.into(),
            file_type: FileType::Dir,
            file_format: Cell::new(FileFormat::default()),
        }))
    }

    pub fn file_path_abs(&self) -> &Path {
        &self.0.file_path
    }

    pub fn file_path(&self) -> &Path {
        crate::relative_path(&self.0.file_path)
    }

    pub fn file_type(&self) -> FileType {
        self.0.file_type
    }

    pub fn file_format(&self) -> FileFormat {
        self.0.file_format.get()
    }

    pub fn set_file_format(&mut self, file_format: FileFormat) {
        self.0.file_format.set(file_format);
    }
}

impl std::fmt::Debug for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            match self.file_type() {
                FileType::Dir => write!(f, "{}:{}", self.file_type(), self.file_path().display()),
                FileType::File => write!(
                    f,
                    "{}<{}>:{}",
                    self.file_type(),
                    self.file_format(),
                    self.file_path().display()
                ),
                _ => unreachable!(),
            }
        } else {
            match self.file_type() {
                FileType::Dir => write!(
                    f,
                    "{}:{}",
                    self.file_type(),
                    crate::relative_path(&self.file_path()).display()
                ),
                FileType::File => write!(
                    f,
                    "{}<{}>:{}",
                    self.file_type(),
                    self.file_format(),
                    crate::relative_path(&self.file_path()).display()
                ),
                _ => unreachable!(),
            }
        }
    }
}

impl PartialEq<FileInfo> for FileInfo {
    fn eq(&self, other: &FileInfo) -> bool {
        if Rc::ptr_eq(&self.0, &other.0) {
            true
        } else {
            self.file_path() == other.file_path()
        }
    }
}

impl Eq for FileInfo {}

impl PartialOrd<FileInfo> for FileInfo {
    fn partial_cmp(&self, other: &FileInfo) -> Option<Ordering> {
        self.file_path().partial_cmp(other.file_path())
    }
}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_path().cmp(other.file_path())
    }
}

impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_path().hash(state);
    }
}

impl HeapSizeOf for FileInfo {
    fn heap_size_of_children(&self) -> usize {
        PathBufHeapSize(&self.0.file_path).heap_size_of_children()
    }
}

#[derive(Debug)]
pub struct Metadata {
    parent: Option<Weak<RefCell<Node>>>,
    index: usize,
    key: Symbol,
    file: Option<FileInfo>,
    span: Option<Box<Span>>,
}

impl Metadata {
    pub(super) fn new() -> Metadata {
        Metadata {
            parent: None,
            index: 0,
            key: Symbol::default(),
            file: None,
            span: None,
        }
    }

    pub fn parent(&self) -> Option<NodeRef> {
        match self.parent {
            Some(ref p) => Some(NodeRef::wrap(p.upgrade().unwrap())),
            None => None,
        }
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub fn set_parent(&mut self, p: Option<&NodeRef>) {
        self.parent = p.map(|p| Rc::downgrade(p.unwrap()));
    }

    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    pub fn set_key(&mut self, key: Cow<str>) {
        self.key = Symbol::from(key);
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn file(&self) -> Option<&FileInfo> {
        self.file.as_ref()
    }

    pub fn set_file(&mut self, file: Option<FileInfo>) {
        self.file = file;
    }

    pub fn span(&self) -> Option<Span> {
        self.span.as_ref().map(|s| **s)
    }

    pub fn set_span(&mut self, span: Option<Span>) {
        self.span = span.map(|s| Box::new(s));
    }

    pub(super) fn detach(&mut self) {
        self.parent = None;
        self.index = 0;
        self.key = Symbol::default();
    }

    pub(super) fn deep_copy(&self) -> Metadata {
        Metadata {
            parent: None,
            index: 0,
            key: Symbol::default(),
            file: self.file.clone(),
            span: self.span.clone(),
        }
    }
}

impl HeapSizeOf for Metadata {
    fn heap_size_of_children(&self) -> usize {
        if self.span.is_some() {
            std::mem::size_of::<Span>()
        } else {
            0
        }
    }
}
