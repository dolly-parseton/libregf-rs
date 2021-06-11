#[cfg(test)]
mod tests {
    use std::path;
    #[test]
    fn it_works() {
        let path = path::PathBuf::from(".test_data/SOFTWARE")
            .canonicalize()
            .unwrap();
        if path.exists() {
            if let Some(p) = path.to_str() {
                let reg_f = libregf_sys::file::RegfFile::open(p);
                if let Ok(f) = reg_f {
                    println!("{:?}", f.root_node().unwrap().get_sub_keys());
                }
            }
        }
        assert_eq!(false, true);
    }
}
