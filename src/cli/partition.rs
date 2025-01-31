use std::{fs::File, num::NonZeroU64, os::unix::fs::MetadataExt, path::PathBuf};

use super::{Error, Kind};

#[derive(Debug)]
pub struct Partition {
    pub active: bool,
    pub kind: Kind,
    pub path: PathBuf,
    pub file: File,
    pub size: NonZeroU64,
}

impl std::str::FromStr for Partition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let active = s.starts_with('+');
        let kv = &s[active.into()..];
        let (kind, path) = kv
            .split_once('=')
            .ok_or(format!("missing '=' in `{}`", kv))?;

        let kind = kind.parse()?;
        let path = PathBuf::from(path);
        let file = File::open(&path)?;
        let size = NonZeroU64::new(file.metadata()?.size())
            .ok_or_else(|| format!("empty file: {:?}", path))?;

        Ok(Self {
            active,
            kind,
            path,
            file,
            size,
        })
    }
}
