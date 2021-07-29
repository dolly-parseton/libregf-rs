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
    //
    pub fn root(&self) -> Result<Key, Box<dyn std::error::Error>> {
        self.file.root_node().map(|k| k.into())
    }
}

pub struct HiveIterator {
    pub hive: Hive,
    state: Vec<u64>,
}

impl HiveIterator {
    pub fn set_lowest(&mut self, current: &Key) {
        self.state.push(0);
        let keys = current.sub_keys();
        if let Ok(false) = keys.as_ref().map(|ref v| v.is_empty()) {
            if let Some(k) = keys.map(|mut v| v.pop()).ok().flatten() {
                self.set_lowest(&k);
                // self.parent = k
            }
        }
    }
    pub fn next_key(&mut self) -> Option<Key> {
        // fn inner(key: &mut Option<Key>, positions: &mut Vec<u64>) {
        //     if depth != positions.len() {
        //         if let Ok(Some(k)) = key.sub_key(positions[positions.len() - 1] as usize) {
        //             inner(k, depth + 1, positions);
        //         } else {
        //         }
        //     }
        // }
        // // Go to current +1 to n-1, if some then return else go up a level and + 1 <recurse until success>
        // let mut path = Vec::new();
        // inner(self.hive.root()?, &mut key, 0, &self.state);
        // None
    }
    pub fn generate_path(&self) -> Result<String, Box<dyn std::error::Error>> {
        fn inner(key: Key, path: &mut Vec<String>, depth: usize, positions: &[u64]) {
            if let Ok(name) = key.name() {
                path.push(name);
                if depth != positions.len() {
                    if let Ok(Some(k)) = key.sub_key(positions[depth] as usize) {
                        inner(k, path, depth + 1, positions);
                    }
                }
            }
        }
        let mut path = Vec::new();
        inner(self.hive.root()?, &mut path, 0, &self.state);
        Ok(path.join("/"))
    }
    pub fn from_hive(hive: Hive) -> Result<Self, Box<dyn error::Error>> {
        let mut iter = Self {
            hive,
            state: Vec::new(),
        };
        if let Some(k) = iter.hive.root()?.sub_key(0)? {
            iter.set_lowest(k);
        }
        println!("{:?}", iter.state);
        println!("{:?}", iter.generate_path());
        // Enumerate all the keys
        Ok(iter)
    }
}

impl Iterator for HiveIterator {
    // we will be counting with usize
    type Item = crate::Key;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
