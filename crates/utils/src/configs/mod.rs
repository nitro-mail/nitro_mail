use serde::de::{DeserializeOwned, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use thiserror::Error;

pub mod dkim;
pub mod domain_configs;
mod duration;
mod type_or_path;
pub use duration::ConfigDuration;
pub use type_or_path::PathOrType;

#[derive(Debug, Error)]
pub enum IOOrToml {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Toml error: {0}")]
    Toml(#[from] toml::de::Error),
}
#[derive(Debug, PartialEq, Eq)]
pub enum ConfigName {
    Name(&'static str),
    Directory,
}
impl Into<PathBuf> for ConfigName {
    fn into(self) -> PathBuf {
        match self {
            ConfigName::Name(name) => PathBuf::from(name),
            _ => panic!("Called Into<PathBuf> on a ConfigName::Directory. This is a bug."),
        }
    }
}
pub trait Config: DeserializeOwned + Default + Serialize + Send + Sync {
    fn config_header() -> Option<&'static str>
    where
        Self: Sized;

    fn config_name() -> ConfigName
    where
        Self: Sized;

    fn load(working_directory: PathBuf) -> Result<Self, IOOrToml>
    where
        Self: Sized,
    {
        if let ConfigName::Name(name) = Self::config_name() {
            let mut config_file = std::fs::File::open(working_directory.join(name))?;
            let mut config_string = String::new();
            config_file.read_to_string(&mut config_string)?;
            Ok(toml::from_str(config_string.as_str())?)
        } else {
            panic!("The default load implementation is only for files, not directories.")
        }
    }
    fn load_from_path(path: PathBuf) -> Result<Self, IOOrToml>
    where
        Self: Sized,
    {
        let mut config_file = std::fs::File::open(path)?;
        let mut config_string = String::new();
        config_file.read_to_string(&mut config_string)?;
        Ok(toml::from_str(config_string.as_str())?)
    }
    fn save(&self, working_directory: PathBuf) -> Result<(), IOOrToml>
    where
        Self: Sized,
    {
        if let ConfigName::Name(name) = Self::config_name() {
            self.save_to_path(working_directory.join(name))
        } else {
            panic!("The default save implementation is only for files, not directories.")
        }
    }
    fn save_to_path(&self, path: PathBuf) -> Result<(), IOOrToml> {
        if path.exists() {
            // Backup the file
            let mut backup_path = path.clone();
            backup_path.set_extension("bak.toml");
            std::fs::copy(&path, &backup_path)?;
        }

        let mut config_file = OpenOptions::new().create(true).write(true).open(path)?;

        let config_string =
            toml::to_string(self).expect("Failed to serialize config. This is a bug.");

        if let Some(header) = Self::config_header() {
            config_file.write_all(b"# ")?;
            config_file.write_all(header.as_bytes())?;
            config_file.write_all(b"\n")?;
        }
        config_file.write_all(config_string.as_bytes())?;
        Ok(())
    }
    fn get_or_save_default(working_directory: PathBuf) -> Result<Self, IOOrToml> {
        if let ConfigName::Name(value) = Self::config_name() {
            let path = working_directory.join(value);
            if path.exists() {
                Self::load_from_path(path)
            } else {
                let config = Self::default();
                config.save_to_path(path)?;
                Ok(config)
            }
        } else {
            panic!("The default get_or_save_default implementation is only for files, not directories.")
        }
    }
}
impl Config for () {
    fn config_header() -> Option<&'static str>
    where
        Self: Sized,
    {
        None
    }

    fn config_name() -> ConfigName
    where
        Self: Sized,
    {
        ConfigName::Name("dont_save.toml")
    }
    fn load(_: PathBuf) -> Result<Self, IOOrToml>
    where
        Self: Sized,
    {
        Ok(())
    }
    fn save(&self, _: PathBuf) -> Result<(), IOOrToml>
    where
        Self: Sized,
    {
        Ok(())
    }
}
macro_rules! tuple_configs {
    ($($T:ident),*) => {
        impl<$($T: Config),*> Config for ($($T),*) {
            fn config_header() -> Option<&'static str>
            where
                Self: Sized,
            {
                None
            }

            fn config_name() -> ConfigName
            where
                Self: Sized,
            {
                ConfigName::Directory
            }
            fn load(working_directory: PathBuf)->Result<Self, IOOrToml> {
                #[allow(non_snake_case)]
                let ( $($T),* ) = (
                    $(
                        $T::load(working_directory.clone())?,
                    )*
                );
                Ok(($($T),*))
            }
            fn load_from_path(path: PathBuf) -> Result<Self, IOOrToml> where Self: Sized,
                {
                    panic!("Load From Path is not implemented for tuples.")
                }
            fn save_to_path(&self, path: PathBuf) -> Result<(), IOOrToml> {
                panic!("Load From Path is not implemented for tuples.")
            }

            fn save(&self, working_directory: PathBuf) -> Result<(), IOOrToml>
            where
                Self: Sized,
            {
                #[allow(non_snake_case)]
                let ( $($T),* ) = self;
                $(
                    $T.save(working_directory.clone())?;
                )*
                Ok(())
            }
            fn get_or_save_default(working_directory: PathBuf) -> Result<Self, IOOrToml> {
                #[allow(non_snake_case)]
                let ( $($T),* ) = (
                    $(
                        $T::get_or_save_default(working_directory.clone())?,
                    )*
                );
                Ok(($($T),*))
            }
        }
    };
}
tuple_configs!(A, B);
tuple_configs!(A, B, C);
tuple_configs!(A, B, C, D);
tuple_configs!(A, B, C, D, E);
