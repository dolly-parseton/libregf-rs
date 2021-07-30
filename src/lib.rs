mod error;
mod hive;
mod kv;
//
pub use hive::Hive;
pub use kv::{Key, Value};
//
use chrono::{DateTime, NaiveDateTime, Utc};
//
pub fn epoch_to_timestamp(sec: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(sec, 0), Utc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;
    fn recurse(keys: &[libregf_sys::key::RegfKey]) {
        for key in keys {
            // println!("{:?}", key);
            println!("name: {:?}", key.get_name());
            println!("ts: {:?}", key.get_last_written());
            for i in 0..key.get_number_of_values().unwrap() {
                if let Ok(value) = key.get_value_by_index(i) {
                    println!("\tname: {:?}", value.get_name());
                    println!("\ttype: {:?}", value.get_type());
                    match value.get_type() {
                        Ok(libregf_sys::value::RegfType::String) => {
                            println!("\tvalue: {:?}", value.get_string())
                        }
                        Ok(libregf_sys::value::RegfType::ExpandableString) => {
                            println!("\t\tvalue: {:?}", value.get_string())
                        }
                        Ok(libregf_sys::value::RegfType::Binary) => {
                            println!("\t\tvalue: {:?}", value.get_binary())
                        }
                        _ => (),
                    }
                }
            }
            if let Ok(ref v) = key.get_sub_keys() {
                recurse(v);
            }
        }
    }
    #[test]
    fn it_works() {
        let path = path::PathBuf::from(".test_data/config/DEFAULT")
            .canonicalize()
            .unwrap();
        if path.exists() {
            if let Some(p) = path.to_str() {
                let reg_f = libregf_sys::file::RegfFile::open(p);
                if let Ok(f) = reg_f {
                    if let Ok(ref keys) = f.root_node().unwrap().get_sub_keys() {
                        recurse(keys);
                    }
                }
            }
        }
    }
    #[test]
    fn it_works2() {
        let dt = std::time::Instant::now();
        let path = path::PathBuf::from(".test_data/config/DEFAULT")
            .canonicalize()
            .unwrap();
        let hive = Hive::from_path(path).unwrap();
        let mut iter = hive.into_iter().unwrap();
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        // println!("{:?}", iter.next());
        for key in iter {
            println!("{:?}", key);
            // std::thread::sleep(std::time::Duration::from_millis(25));
        }
        println!("{:?}", dt.elapsed());
        assert_eq!(false, true);
    }
}
