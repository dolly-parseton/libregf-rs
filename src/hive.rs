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
    pub fn seek_lowest(&mut self, current: &Key) {
        if let Ok(true) = current.sub_keys_len().map(|l| l != 0) {
            if let Ok(Some(k)) = current.sub_key(0) {
                self.state.push(0);
                self.seek_lowest(&k);
            }
        }
    }
    pub fn next_key(&mut self) -> Option<Result<Key, Box<dyn std::error::Error>>> {
        fn get_current_parent(
            current: &mut Option<Key>,
            key: Key,
            depth: usize,
            positions: &[u64],
        ) -> Option<Key> {
            if depth != positions.len() - 1 && positions.len() > 0 {
                if let Ok(Some(k)) = key.sub_key(positions[depth] as usize) {
                    // println!("{}, {}, {:?}", depth, positions[depth], k);
                    get_current_parent(current, k, depth + 1, positions);
                }
            } else {
                *current = Some(key);
            }
            None
        }
        fn inner(iter: &mut HiveIterator, key: &mut Option<Key>) -> Result<(), ()> {
            // See if there is another key ahead of the current
            if let Some(Ok(Some(k))) = key
                .as_ref()
                .map(|k| k.sub_key((iter.state[iter.state.len() - 1] + 1) as usize))
            {
                let len = iter.state.len();
                println!("INNER BEFORE: {:?}, {:?}", iter.state, k);
                iter.state[(len - 1) as usize] = iter.state[iter.state.len() - 1] + 1;
                iter.seek_lowest(&k);
                println!("INNER AFTER: {:?}, {:?}", iter.state, k);
                *key = Some(k);
            } else {
                // Go back up one and return states current.
                iter.state.pop();
                println!("INNER UP ONE: {:?}, {:?}", iter.state, key);
                get_current_parent(key, iter.hive.root().map_err(|_| ())?, 0, &iter.state);
                // inner(iter, key)?;
                if let Some(Ok(Some(k))) = key
                    .as_ref()
                    .map(|k| k.sub_key((iter.state[iter.state.len() - 1]) as usize))
                {
                    *key = Some(k);
                }
            }
            Ok(())
        }
        let mut key = None;
        get_current_parent(&mut key, self.hive.root().ok()?, 0, &self.state);
        // println!("{:?}", key);
        inner(self, &mut key);
        match (key.as_mut(), self.generate_path_parts()) {
            (Some(mut k), Ok(v)) => k.path_parts = v,
            _ => (),
        }
        key.map(Ok)
    }
    pub fn generate_path_parts(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
        Ok(path)
    }
    pub fn from_hive(hive: Hive) -> Result<Self, Box<dyn error::Error>> {
        let mut iter = Self {
            hive,
            state: Vec::new(),
        };
        if let Some(k) = iter.hive.root()?.sub_key(0)? {
            iter.set_lowest(&k);
        }
        println!("{:?}", iter.state);
        println!("{:?}", iter.generate_path_parts());
        // Enumerate all the keys
        Ok(iter)
    }
}

impl Iterator for HiveIterator {
    // we will be counting with usize
    type Item = Result<crate::Key, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_key()
    }
}
