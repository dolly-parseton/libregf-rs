//
use crate::kv::Key;
use libregf_sys::file::RegfFile;
use std::path;
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
    pub fn root(&self) -> Result<Key, Box<dyn std::error::Error>> {
        self.file.root_node().map(|k| k.into())
    }
}

impl IntoIterator for Hive {
    type Item = Result<crate::Key, Box<dyn std::error::Error>>;
    type IntoIter = HiveIterator;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::from_hive(self)
    }
}

pub struct HiveIterator {
    hive: Hive,
    state: Vec<usize>,
    parents: Vec<Key>,
    started: bool,
    done: bool,
}

impl HiveIterator {
    pub fn seek_lowest(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        fn inner(iter: &mut HiveIterator) -> Result<(), Box<dyn std::error::Error>> {
            if let Some(p) = iter.state.last() {
                if let Some(k) = match iter.parents.last() {
                    Some(k) => k.sub_key(*p)?,
                    None => iter.hive.root()?.sub_key(*p)?,
                } {
                    if k.sub_keys_len()? != 0 {
                        iter.state.push(0);
                        iter.parents.push(k);
                        inner(iter)?;
                    }
                }
            }
            Ok(())
        }
        inner(self)
    }
    pub fn next_key(&mut self) -> Result<Option<Key>, Box<dyn std::error::Error>> {
        match (self.started, self.state.is_empty(), self.done) {
            (false, true, _) => {
                if self.hive.root()?.sub_keys_len()? != 0 {
                    self.state.push(0);
                    self.seek_lowest()?;
                }
                self.started = true;
            }
            (true, true, false) => {
                self.done = true;
                return self.hive.root().map(|mut k| {
                    k.path_parts.push("ROOT".into());
                    Some(k)
                });
            }
            (true, true, true) => {
                return Ok(None);
            }
            _ => (),
        }
        // Store current
        let current = match match (self.parents.last(), self.state.last()) {
            (Some(k), Some(p)) => k.sub_key(*p)?,
            (None, Some(p)) => self.hive.root()?.sub_key(*p)?,
            _ => None,
        } {
            Some(mut k) => {
                self.set_key_path(&mut k)?;
                Some(k)
            }
            None => None,
        };
        // Next key
        match match (self.parents.last(), self.state.last()) {
            (Some(k), Some(p)) => *p + 1 < k.sub_keys_len()?,
            (None, Some(p)) => *p + 1 < self.hive.root()?.sub_keys_len()?,
            _ => false,
        } {
            true => {
                self.state.last_mut().map(|p| {
                    *p += 1;
                    p
                });
                self.seek_lowest()?;
            }
            false => {
                self.state.pop();
                self.parents.pop();
            }
        };
        Ok(current)
    }
    pub fn set_key_path(&self, current: &mut Key) -> Result<(), Box<dyn std::error::Error>> {
        current.path_parts.push(self.hive.root()?.name()?);
        for parent in &self.parents {
            current.path_parts.push(parent.name()?);
        }
        current.path_parts.push(current.name()?);
        Ok(())
    }
    pub fn from_hive(hive: Hive) -> Self {
        Self {
            hive,
            state: Vec::new(),
            parents: Vec::new(),
            started: false,
            done: false,
        }
    }
}

impl Iterator for HiveIterator {
    type Item = Result<crate::Key, Box<dyn std::error::Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_key() {
            Ok(Some(k)) => Some(Ok(k)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
