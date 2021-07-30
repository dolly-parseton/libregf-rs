use chrono::{DateTime, Utc};
use libregf_sys::{key::RegfKey, value::RegfValue};
use std::{convert::TryFrom, error};

#[derive(Debug)]
pub struct Key {
    pub path_parts: Vec<String>,
    inner: RegfKey,
}

impl From<RegfKey> for Key {
    fn from(inner: RegfKey) -> Self {
        Self {
            inner,
            path_parts: Vec::new(),
        }
    }
}

impl Key {
    pub fn from_parents(
        inner: RegfKey,
        parents: &[RegfKey],
    ) -> Result<Self, Box<dyn error::Error>> {
        let mut path_parts = Vec::new();
        for part in parents.iter().map(|p| p.get_name()) {
            path_parts.push(part?);
        }
        Ok(Self { path_parts, inner })
    }

    pub fn sub_keys(&self) -> Result<Vec<Self>, Box<dyn error::Error>> {
        self.inner
            .get_sub_keys()
            .map(|mut v| v.drain(..).map(|k| k.into()).collect())
            .map_err(|e| e.into())
    }

    pub fn sub_key(&self, i: usize) -> Result<Option<Self>, Box<dyn error::Error>> {
        if let Ok(true) = self.sub_keys_len().map(|l| i < l) {
            Ok(self.inner.get_sub_key(i).ok().map(|k| k.into()))
        } else {
            Ok(None)
        }
    }

    pub fn sub_keys_len(&self) -> Result<usize, Box<dyn error::Error>> {
        self.inner.get_sub_keys_len().map_err(|e| e.into())
    }

    pub fn name(&self) -> Result<String, Box<dyn error::Error>> {
        self.inner.get_name().map_err(|e| e.into())
    }

    pub fn last_modified(&self) -> Result<DateTime<Utc>, Box<dyn error::Error>> {
        use std::convert::TryInto;
        Ok(crate::epoch_to_timestamp(
            self.inner.get_last_written()?.try_into()?,
        ))
    }
}

//

#[derive(Debug)]
pub struct Value {
    pub value: RegfValue,
}

impl From<RegfValue> for Value {
    fn from(value: RegfValue) -> Self {
        Self { value }
    }
}

impl Value {}
