pub fn from_root_path(file_path: &str) -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf();
    path.push(file_path);
    path
}

pub async fn from_root_file(file_path: &str) -> Vec<u8> {
    let path = from_root_path(file_path);
    println!("path: {:?}", path);
    tokio::fs::read(path).await.unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
