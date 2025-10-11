use docsmith_base::result::DocsmithResult;
use relative_path::RelativePathBuf;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::rc::Rc;

pub type FilePath = RelativePathBuf;

// Platform abstraction layer used to decouple logic from the underlying platform
pub trait Pal: Debug + 'static {
    /// Read a file, the path is relative to the base directory
    fn read_file(&self, path: &FilePath) -> DocsmithResult<Box<dyn Read + 'static>>;

    /// Create a file to a string, the path is relative to the base directory
    fn create_file(&self, path: &FilePath) -> DocsmithResult<Box<dyn Write>>;

    /// Create a directory (including parent directories), the path is relative to the base directory
    fn create_directory_all(&self, path: &FilePath) -> DocsmithResult<()>;

    /// Remove a directory (including _all_ content), the path is relative to the base directory
    fn remove_directory_all(&self, path: &FilePath) -> DocsmithResult<()>;
}

pub type PalBox = Rc<dyn Pal>;
