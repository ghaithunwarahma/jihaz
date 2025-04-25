use std::{path::{Path, PathBuf}, fs::OpenOptions};

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DataFormat {
    /// Json format via serde_json.
    Json,
    /// Pretty Json format via serde_json.
    JsonPretty,
    /// Default binary format via bincode.
    Bincode,
}

pub mod ser {
    //! Note that using dyn Path (i.e. P: AsRef\<Path\>) will cause a whole lot of problems.
    //! So I just used &Path here, and impl AsRef\<Path\> in FileManager as there are no dynamic type arguments there.
    
    use std::{path::Path, fs::OpenOptions};
    
    use super::{DataFormat, SerdeError};

    /// Serializes T into path
    pub fn serialize_into_path<T>(value: &T, path: &Path, format: DataFormat) -> Result<(), SerdeError> 
    where
        T: serde::Serialize,
    {
        let parent = path.parent().unwrap();
        if !parent.is_dir() {
            std::fs::create_dir_all(parent)
            .expect(format!("Problem creating dir all for {:?}", path).as_str());
        }
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        serialize_into::<T, std::fs::File>(value, &mut file, format)
    }

    /// Serializes an object directly into a `Writer` using the default configuration.
    /// Serializes T into writer
    pub fn serialize_into<T, W>(value: &T, writer: &mut W, format: DataFormat) -> Result<(), SerdeError> 
    where
        T: serde::Serialize,
        W: std::io::Write,
    {
        let result = match format {
            // does this assumes that the data struct needs to be T and not &[u8]
            // I think so,
            DataFormat::Json => serde_json::to_writer(writer, value)?,
            // does this assumes that the data struct needs to be T and not &[u8]?
            // I think so,
            DataFormat::JsonPretty => serde_json::to_writer_pretty(writer, value)?,
            DataFormat::Bincode => bincode::serialize_into(writer, value)?,
        };
        Ok(result)
    }

    /// Serializes a serializable object into a `Vec` of bytes.
    pub fn serialize<T>(value: &T, format: DataFormat) -> Result<Vec<u8>, SerdeError> 
    where
        T: serde::Serialize,
    {
        let result = match format {
            DataFormat::Json => serde_json::to_vec(value)?,
            DataFormat::JsonPretty => serde_json::to_vec_pretty(value)?,
            DataFormat::Bincode => bincode::serialize(value)?,
        };
        Ok(result)
    }

    /// Serializes a serializable object into a `String` using serde_json.
    pub fn serialize_to_string<T>(value: &T, format: DataFormat) -> Result<String, SerdeError> 
    where
        T: serde::Serialize,
    {
        let result = match format {
            DataFormat::Json => serde_json::to_string(value)?,
            DataFormat::JsonPretty => serde_json::to_string_pretty(value)?,
            DataFormat::Bincode => panic!("bincode cannot serialize to String"),
        };
        Ok(result)
    }

    /// Returns the size that an object would be if serialized using support::serde::ser::serialize.
    pub fn serialized_size<T: ?Sized>(value: &T, format: DataFormat) -> Result<u64, SerdeError>
    where
        T: serde::Serialize,
    {
        let result = match format {
            DataFormat::Json => {
                serde_json::to_string(value).map(|s| s.len() as u64)?
            },
            DataFormat::JsonPretty => {
                serde_json::to_string_pretty(value).map(|s| s.len() as u64)?
            }
            DataFormat::Bincode => bincode::serialized_size(value)?,
        };
        Ok(result)
    }
}

pub mod de {
    //! Note that using AsRef Path (i.e. P: AsRef\<Path\>) will cause a whole lot of problems.
    //! So I just used &Path here, and impl AsRef\<Path\> in FileManager as there are no dynamic type arguments there.
    
    use std::path::{Path, PathBuf};
    use std::io::BufReader;
        
    use super::load_file;
    
    use super::{DataFormat, SerdeError};

    /// Deserialize T from path.
    /// Deserializing into a type that implements DeserializeOwned from reader, using the expected data format.
    // Cannot use impl AsRef<Path>, because when we have explicit generic type the impl cannot be used.
    pub fn deserialize_from_path<T>(path: &Path, format: DataFormat) -> Result<T, SerdeError> 
    where
        T: serde::de::DeserializeOwned,
    {
        let file = BufReader::new(load_file(path)?);
        // let attempt_other_incoding = true;

        let res = deserialize_from::<T, BufReader<std::fs::File>>(file, format);
        
        // if attempt_other_incoding {
            if let Err(e) = &res {
                // I decided I should not attempt other encoding.. as we can get this error in terminal: 
                // <*> failed deserializing Json, trying Bincode. error is serde_json error: missing field `panes` at line 1 column 19085. path: "..."
                // memory allocation of 3617855283104265015 bytes failed
                // zsh: abort      cargo run.

                // so I'll do this instead.
                eprintln!("<*> [suppor::serde] failed deserializing {:?}. error is {:}. path: {:?}", format, e, path);

                // let (msg, format) = match format {
                //     DataFormat::Json | DataFormat::JsonPretty => ("Json, trying Bincode", DataFormat::Bincode),
                //     DataFormat::Bincode => ("Bincode, trying Json", DataFormat::Json),
                // };
                // eprintln!("<*> failed deserializing {}. error is {:}. path: {:?}", msg, e, path);

                // let file = BufReader::new(load_file(path)?);
                // return deserialize_from::<T, BufReader<std::fs::File>>(file, format);
            }
        // }
        res
    }

    /// Deserializes an object directly from a `Read`er using default configuration.
    /// Deserializing into a type that implements DeserializeOwned from reader, whether its is a json file or binary file.
    pub fn deserialize_from<T, R>(reader: R, format: DataFormat) -> Result<T, SerdeError> 
    where
        T: serde::de::DeserializeOwned,
        R: std::io::Read,
    {
        let result = match format {
            DataFormat::Json | DataFormat::JsonPretty => {
                serde_json::from_reader::<R, T>(reader)?
            }
            DataFormat::Bincode => bincode::deserialize_from::<R, T>(reader)?,
        };
        Ok(result)
    }

    /// Deserializes a slice of bytes into an instance of `T` using the default configuration.
    pub fn deserialize<T>(bytes: &[u8], format: DataFormat) -> Result<T, SerdeError> 
    where
        T: serde::de::DeserializeOwned,
    {
        let result = match format {
            DataFormat::Json | DataFormat::JsonPretty => serde_json::from_slice(bytes)?,
            DataFormat::Bincode => bincode::deserialize(bytes)?,
        };
        Ok(result)
    }

    /// Deserializes a slice of bytes into an instance of `T` using the default configuration.
    pub fn deserialize_from_str<T>(s: &str, format: DataFormat) -> Result<T, SerdeError> 
    where
        T: serde::de::DeserializeOwned,
    {
        let result = match format {
            DataFormat::Json | DataFormat::JsonPretty => serde_json::from_str(s)?,
            DataFormat::Bincode => panic!("bincode cannot deserialize from str"),
        };
        Ok(result)
    }

    /// Deserializing objects in a given directory
    pub fn deserialize_from_directory<T>(
        dir: &Path, levels: Option<usize>, format: DataFormat, extension: &str
    ) -> Result<Vec<T>, SerdeError> 
    where
        T: serde::de::DeserializeOwned,
    {
        deserialize_from_directory_map(dir, levels, format, extension, |value, _| value)
    }

    /// Deserializing objects in a given directory, mapping each one with its path
    pub fn deserialize_from_directory_map<T, V>(
        dir: &Path,
        levels: Option<usize>,
        format: DataFormat, 
        extension: &str, 
        map_obj: impl Fn(T, PathBuf) -> V,
    ) -> Result<Vec<V>, SerdeError> 
    where
        T: serde::de::DeserializeOwned,
    {
        let mut children = Vec::new();

        let name_of_file = std::any::type_name::<T>();

        let mut paths = Vec::new();

        super::paths_in_dir(1, levels, dir, extension, &mut paths);

        for path in paths.drain(..) {
            match crate::serde::de::deserialize_from_path::<T>(&path, format) {

                Ok(child_file) => children.push(map_obj(child_file, path)),
                Err(e) => {
                    eprintln!("Error deserializing {}, path: {:?}", name_of_file, &path);
                    return Err(e);
                }
            }
        }

        if children.len() == 0 {
            eprintln!("Did not find any {}", name_of_file);
        }
        Ok(children)
    }
}

/// Loads file with read permission
pub fn load_file(path: &Path) -> Result<std::fs::File, std::io::Error> {
    OpenOptions::new().read(true).open(path)
}

pub fn remove_file(path: impl AsRef<Path>, name: &str) {
    let path = path.as_ref();
    if let Err(e) = std::fs::remove_file(path) {
        if !matches!(e.kind(), std::io::ErrorKind::NotFound) {
            eprintln!("Failed to remove {} at path ({:?}): {}", name, path, e);
        }
    }
}

pub fn create_dir_all(path: impl AsRef<Path>) {
    let path = path.as_ref();
    if let Err(e) = std::fs::create_dir_all(path) {
        if !matches!(e.kind(), std::io::ErrorKind::AlreadyExists) {
            eprintln!("Failed to crate_dir_all for directory ({:?}): {}", path, e);
        }
    }
}

pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    let (from, to) = (from.as_ref(), to.as_ref());
    if let Err(e) = std::fs::copy(from, to) {
        eprintln!("Failed to copy ({:?}) to ({:?}): {}", from, to, e);
    }
}

pub fn paths_in_dir(
    mut current_level: usize, 
    levels: Option<usize>, 
    dir: &Path, 
    extension: &str,
    paths: &mut Vec<PathBuf>
) {
    if let Ok(res) = std::fs::read_dir(dir) {
        let mapped = res.map(|res| res.map(|e| e.path()));
        if let Ok(mut entries) = mapped.collect::<Result<Vec<_>, std::io::Error>>() {
            for entry in entries.drain(..) {
                if entry.is_dir() {
                    if levels.map(|levels| levels == current_level).unwrap_or(false) {
                        continue;
                    } else {
                        current_level += 1;
                        paths_in_dir(current_level, levels, &entry, extension, paths);
                    }
                } else {
                    // eprintln!("found file {:?}", &entry);
                    let this_extension = entry.extension().and_then(|e| e.to_str());
                    if this_extension.map(|e| e == extension).unwrap_or_default() {
                        paths.push(entry);
                    }
                }
            }
        }
    }
}


#[derive(thiserror::Error, Debug)]
pub enum SerdeError {
    #[error("std io error: {0}")]
    StdIO(std::io::Error),
    #[error("serde_json error: {0}")]
    SerdeJson(serde_json::error::Error),
    #[error("bincode error: {0}")]
    Bincode(bincode::Error),
}

macro_rules! from {
    ($variant:ident, $path:path) => {
        impl From<$path> for SerdeError {
            fn from(data: $path) -> SerdeError {
                SerdeError::$variant(data)
            }
        }
    }
}

from!(StdIO, std::io::Error);
from!(SerdeJson, serde_json::error::Error);
from!(Bincode, bincode::Error);