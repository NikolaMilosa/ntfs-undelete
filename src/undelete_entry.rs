use std::fmt::Display;

use mft::{
    attribute::{MftAttributeContent, MftAttributeType},
    MftEntry,
};

#[derive(Debug)]
pub struct UndeleteEntry {
    pub filename: String,
    pub record_number: u64,
    pub is_allocated: bool,
}

impl From<MftEntry> for UndeleteEntry {
    fn from(value: MftEntry) -> Self {
        let file_attribute = value
            .iter_attributes_matching(Some(vec![MftAttributeType::FileName]))
            .find_map(|attr| match attr {
                Ok(attribute) => match attribute.data {
                    MftAttributeContent::AttrX30(filename) => Some(filename),
                    _ => None,
                },
                Err(_) => None,
            });

        Self {
            filename: match file_attribute {
                Some(attr) => attr.name,
                None => "".to_string(),
            },
            is_allocated: value.is_allocated(),
            record_number: value.header.record_number,
        }
    }
}

impl Display for UndeleteEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.filename))
    }
}
