//
use crate::kv::Key;
use libregf_sys::{file::RegfFile, key::RegfKey};
use std::{error, path};
//
#[derive(Debug)]
pub struct Hive {
    pub file: RegfFile,
}

impl Hive {
    //
    pub fn from_path<P: AsRef<path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        match path.as_ref().to_str() {
            Some(p) => Ok(Self {
                file: RegfFile::open(p)?,
            }),
            None => Err("Could not resolve path string.".into()),
        }
    }
    //
    pub fn into_iter(self) -> Result<HiveIterator, Box<dyn std::error::Error>> {
        HiveIterator::from_hive(self)
    }
}

pub struct HiveIterator {
    pub hive: Hive,
    // current_parent_path_parts: Vec<Key>,
    keys: Vec<Key>,
}

impl HiveIterator {
    fn recurse_keys(current_key: RegfKey, iter: &mut Self) -> Result<(), Box<dyn error::Error>> {
        for key in current_key.get_sub_keys()?.drain(..) {
            // current_parent_path_parts.push(&key);
            Self::recurse_keys(key, iter)?;
        }
        iter.keys.push(current_key.into());
        Ok(())
    }
    pub fn from_hive(hive: Hive) -> Result<Self, Box<dyn error::Error>> {
        let mut iter = Self {
            hive,
            // current_parent_path_parts: Vec::new(),
            keys: Vec::new(),
        };
        Self::recurse_keys(iter.hive.file.root_node()?, &mut iter)?;
        println!("{:?}", iter.keys.len());
        // Enumerate all the keys
        Ok(iter)
    }
}

impl Iterator for HiveIterator {
    // we will be counting with usize
    type Item = crate::Key;

    fn next(&mut self) -> Option<Self::Item> {
        self.keys.pop().map(|k| k.into())
    }
}
