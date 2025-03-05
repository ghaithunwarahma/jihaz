use std::{path::{Path, PathBuf}, sync::Arc};
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use directories::BaseDirs;
// use druid::Data;

use crate::serde::{DataFormat, SerdeError};

pub fn combile_path_with_stem(path: impl AsRef<Path>, stem: impl AsRef<std::ffi::OsStr>) -> PathBuf {
    let stem = Path::new(stem.as_ref());
    [path.as_ref(), stem].iter().collect()
}

pub fn data_dir_path() -> PathBuf {
    BaseDirs::new()
        .map(|b| b.data_dir().to_owned())
        .unwrap()
}

pub fn data_dir_file_path(relative_path: impl AsRef<Path>) -> PathBuf {
    BaseDirs::new()
        .as_ref()
        .map(|b| b.data_dir())
        .map(|dir| [dir, relative_path.as_ref()].iter().collect())
        .unwrap()
}

/// The directory containing the application data. 
/// Can be the platform specific (the default), or a external directory.
#[derive(Clone, Default, Debug)]
pub struct DataDirectory {
    pub use_external_data_directory: bool, 
    pub external_data_directory: FilePath,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataDirectoryFile {
    use_external_data_directory: bool, 
    external_data_directory: Option<String>,
}

impl DataDirectory {
    pub fn child_path(&self, relative_path: impl AsRef<Path>) -> PathBuf {
        if self.use_external_data_directory {
            self.external_data_directory
                .path()
                .map(|dir| [dir, relative_path.as_ref()].iter().collect())
                .unwrap()
        } else {
            data_dir_file_path(relative_path)
        }
    }

    pub fn path(&self) -> PathBuf {
        if self.use_external_data_directory {
            self.external_data_directory.to_path_buf().unwrap()
        } else {
            data_dir_path()
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct FilePath(pub Option<Arc<Path>>);

impl FilePath {
    /// Includes the extension.
    pub fn file_name(&self) -> Option<String> {
        self.0
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(std::ffi::OsStr::to_str)
            .map(Into::into)
    }

    /// Excludes file extension
    pub fn file_stem(&self) -> Option<String> {
        self.0
            .as_ref()
            .and_then(|p| p.file_stem())
            .and_then(std::ffi::OsStr::to_str)
            .map(Into::into)
    }

    pub fn parent_dir_name(&self) -> Option<String> {
        self.0
            .as_ref()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_stem())
            .and_then(std::ffi::OsStr::to_str)
            .map(Into::into)
    }

    pub fn parent_dir(&self) -> Option<&Path> {
        self.0
            .as_ref()
            .and_then(|p| p.parent())
    }

    /// Returns `true` if the path points at an existing entity.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file.
    ///
    /// If you cannot access the metadata of the file, e.g. because of a
    /// permission error or broken symbolic links, this will return `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use jihaz_primal::file_man::FilePath;
    /// assert!(!FilePath::from(PathBuf::from("does_not_exist.txt")).exists());
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [`fs::metadata`].
    pub fn exists(&self) -> bool {
        self.0.as_ref().map(|path| path.exists()).unwrap_or(false)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn path(&self) -> Option<&Path> {
        use std::ops::Deref;
        self.0.as_ref().map(|path| path.deref())
    }

    pub fn to_path_buf(&self) -> Option<PathBuf> {
        self.0.as_ref().map(|p| p.to_path_buf())
    }

    pub fn to_string(&self) -> Option<String> {
        self.0
            .as_ref()
            .map(|path| path.as_os_str())
            .and_then(std::ffi::OsStr::to_str)
            .map(ToString::to_string)
    }
}

impl<T: Into<PathBuf>> From<T> for FilePath {
    fn from(data: T) -> FilePath {
        FilePath(Some(data.into().into()))
    }
}

// conflicting implementation with the above one
// impl From<Option<String>> for FilePath {
//     fn from(data: Option<String>) -> FilePath {
//         FilePath(data.map(Into::into))
//     }
// }

// impl From<&Path> for FilePath {
//     fn from(data: &Path) -> FilePath {
//         FilePath(Some(data.into_bath_buf().into()))
//     }
// }

impl std::ops::Deref for FilePath {
    type Target = Option<Arc<Path>>;

    fn deref(&self) -> &Option<Arc<Path>> {
        &self.0
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub enum FilePath {
//     Concrete(PathBuf),
//     Dynamic,
// }

// pub trait FileManager<'de>: Sized {
    // type File: Into<Self> + Serialize + Deserialize<'de>;
    // type FileOld: Into<Self::File> + Serialize + Deserialize<'de>;

/// The path is either static or saved internally.
/// 
/// Should rename this to FileManagerWithPath
pub trait FileManagerStatic: FileManager {
    
    fn file_path(&self) -> PathBuf;

    /// Saving file using the path set in the method FileManager::file_path
    fn save(&self) -> Result<(), SerdeError> {
        let path = self.file_path();
        self.save_to(&path)
    }

    /// Loading file using the path set in the method FileManager::file_path
    fn load(&mut self) -> Result<(), SerdeError> {
        let path = self.file_path();
        self.load_from(&path)
    }
}

/// file_stem is only needed when this Self is a Child of a FileManagerChildren implementation.
pub trait FileManager: From<Self::File> + Clone + Sized {

    /// Can be set to the same type as Self if Self implements Serialize + Deserialize
    type File: Serialize + DeserializeOwned + From<Self> + From<Self::FileOld>;
    
    /// Can be set to the same type as Self::File if not needed
    type FileOld: Serialize + DeserializeOwned;

    // should call this io_format
    fn data_format(&self) -> DataFormat;

    /// Includes the file extension
    /// Required when this Self is a Child of a FileManagerChildren implementation.
    /// Warning: That method requires that file_stem is not empty!
    fn file_name(&self) -> String;
    
    // /// Excludes the file extension
    // fn file_stem(&self) -> String {
    //     "".to_string()
    // }

    /// Can be used to save file_path and/or file_name to file.
    /// The default is we do nothing.
    /// Called within load_from so that we get to do set our post loading things without having to rewrite the load_from method.
    fn post_loading(&mut self, _file_path: impl AsRef<Path>) {}

    fn save_to(&self, path: impl AsRef<Path>) -> Result<(), SerdeError> {
        let ft = self.data_format();
        let file = self.clone().into();
        crate::serde::ser::serialize_into_path::<Self::File>(&file, path.as_ref(), ft)
    }

    fn load_from(&mut self, path: impl AsRef<Path>) -> Result<(), SerdeError> {
        let ft = self.data_format();
        match crate::serde::de::deserialize_from_path::<Self::File>(path.as_ref(), ft) {
            Ok(file) => *self = file.into(),
            Err(e) => return Err(e),
        }
        self.post_loading(path);
        Ok(())
    }

    /// When you want control over how the file is loaded
    fn load_from_special(
        &mut self, 
        path: impl AsRef<Path>, 
        load_fn: impl FnOnce(&mut Self, Self::File)
    ) -> Result<(), SerdeError> {
        let ft = self.data_format();
        match crate::serde::de::deserialize_from_path::<Self::File>(path.as_ref(), ft) {
            Ok(file) => load_fn(self, file),
            Err(e) => return Err(e),
        }
        self.post_loading(path);
        Ok(())
    }
}

pub trait FileManagerChildrenStatic: FileManagerChildren {

    fn children_directory() -> PathBuf;

    fn save_children(children: &Vec<Self::Child>) -> Result<(), SerdeError> {
        let children_dir = Self::children_directory();
        Self::save_children_to(&children_dir, children)
    }

    /// Loading children from static directory
    fn load_children(data_format: DataFormat, extension: &str) -> Result<Vec<Self::Child>, SerdeError> {
        let children_dir = Self::children_directory();
        Self::load_children_from(&children_dir, data_format, extension)
    }

    /// Loading children from static directory
    fn load_children_with(
        data_format: DataFormat, 
        extension: &str,
        with: impl Fn(&mut Self::Child, &Path),
    ) -> Result<Vec<Self::Child>, SerdeError> {
        let children_dir = Self::children_directory();
        Self::load_children_from_with(&children_dir, data_format, extension, with)
    }
}

pub trait FileManagerChildren {

    type Child: FileManager;

    /// The type name of this file, the general name of file, i.e. Dictionary, Theme, Notebook etc,
    /// not the file name or stem.
    /// 
    /// File type name.
    /// 
    fn name_of_file() -> &'static str;

    // fn child_extension() -> &'static str;

    fn save_children_to(children_dir: impl AsRef<Path>, children: &Vec<Self::Child>) -> Result<(), SerdeError> {
        for child in children.iter() {
            let mut child_path = children_dir.as_ref().to_owned();
            let child_name = child.file_name();
            assert!(
                child_name.len() > 0, 
                "FileManager::file_name for {} should be specified. found \"\".", 
                Self::name_of_file()
            );
            child_path.push(child.file_name());
            child.save_to(&child_path)?;
        }
        Ok(())
    }

    fn load_children_from(
        children_dir: impl AsRef<Path>,
        data_format: DataFormat,
        extension: &str
    ) -> Result<Vec<Self::Child>, SerdeError> {
        Self::load_children_from_with(children_dir, data_format, extension, |_, _| {})
    }

    /// Loading children from given directory
    fn load_children_from_with(
        children_dir: impl AsRef<Path>,
        data_format: DataFormat,
        extension: &str,
        with: impl Fn(&mut Self::Child, &Path),
    ) -> Result<Vec<Self::Child>, SerdeError> {
        let mut children = Vec::new();

        let name_of_file = Self::name_of_file();
        // let extension = Self::child_extension();
        
        if let Ok(res) = std::fs::read_dir(children_dir) {
            let mapped = res.map(|res| res.map(|e| e.path()));
            if let Ok(entries) = mapped.collect::<Result<Vec<_>, std::io::Error>>() {
                for entry in entries.iter() {
                    // entry is &PathBuf
                    if entry.is_dir() {
                        if let Ok(res) = std::fs::read_dir(entry) {
                            let mapped = res.map(|res| res.map(|e| e.path()));
                            if let Ok(entries) = mapped.collect::<Result<Vec<_>, std::io::Error>>() {
                                for entry in entries.iter() {
                                    let this_extension = entry.extension().and_then(|e| e.to_str());
                                    if this_extension.map(|e| e == extension).unwrap_or_default() {
                                        // note that this load method modifies the child to have the name of its directory,
                                        // so if the containing directory changes, so does the name of the .user_child file,
                                        // and so does the name field inside the .user_child file
                                        // eprintln!("found entry with path {:?}", &entry);

                                        match crate::serde::de::deserialize_from_path::<<Self::Child as FileManager>::File>(entry, data_format) {
                                            Ok(child_file) => {
                                                let mut child: Self::Child = child_file.into();
                                                with(&mut child, entry.as_path());
                                                children.push(child);
                                            }
                                            Err(e) => {
                                                eprintln!("Error loading {}, path: {:?}", name_of_file, &entry);
                                                return Err(e);
                                            }
                                        }
                                    } else {
                                        tracing::info!("entry doesn't match extension {} path {:?}", extension, &entry);
                                    }
                                }
                            }
                        }
                    } else {
                        let this_extension = entry.extension().and_then(|e| e.to_str());
                        if this_extension.map(|e| e == extension).unwrap_or_default() {
                            // bc of this i should not have two libs in one folder
                            // children.push(crate::serde::de::deserialize_from_path::<<Self::Child as FileManager>::File>(entry)?);
                            match crate::serde::de::deserialize_from_path::<<Self::Child as FileManager>::File>(entry, data_format) {
                                Ok(child_file) => {
                                    let mut child: Self::Child = child_file.into();
                                    with(&mut child, entry.as_path());
                                    children.push(child);
                                }
                                Err(e) => {
                                    eprintln!("Error loading {}, path: {:?}", name_of_file, &entry);
                                    return Err(e);
                                }
                            }
                        }
                    }
                }
                if children.len() > 0 {
                    // self.children = Arc::new(children);
                    // self.select_child = self.select_child.min(self.children.len());
                    // let child = self.children.get(self.select_child).unwrap();
                    // self.select_di = self.select_di.min(child.dis.len());
                } else {
                    eprintln!("Did not find any {}", name_of_file);
                }
            }
        }
        // Ok((children, children_path_and_stems))
        Ok(children)
    }
}

impl From<DataDirectoryFile> for DataDirectory {
    fn from(data: DataDirectoryFile) -> DataDirectory {
        DataDirectory {
            use_external_data_directory: data.use_external_data_directory,
            external_data_directory: FilePath(data.external_data_directory.map(PathBuf::from).map(Into::into)),
        }
    }
}

impl From<DataDirectory> for DataDirectoryFile {
    fn from(data: DataDirectory) -> DataDirectoryFile {
        DataDirectoryFile {
            use_external_data_directory: data.use_external_data_directory,
            external_data_directory: data.external_data_directory.to_string(),
        }
    }
}

impl From<&DataDirectory> for DataDirectoryFile {
    fn from(data: &DataDirectory) -> DataDirectoryFile {
        DataDirectoryFile {
            use_external_data_directory: data.use_external_data_directory,
            external_data_directory: data.external_data_directory.to_string(),
        }
    }
}