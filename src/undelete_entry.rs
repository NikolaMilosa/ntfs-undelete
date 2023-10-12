use std::fmt::Display;

#[derive(Debug)]
pub struct UndeleteEntry {
    pub name: String,
    pub inode: u64,
    pub dir: String,
}

impl UndeleteEntry {
    pub fn get_full_path(&self) -> String {
        format!(
            "{}{}",
            match self.dir.as_str() {
                "/" => "".to_string(),
                rest => format!("{}/", &rest[1..]),
            },
            self.name
        )
    }
}

impl Display for UndeleteEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} - Full path: {}",
            self.name,
            self.get_full_path()
        ))
    }
}
