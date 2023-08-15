use std::fmt::Display;

use mft::{
    attribute::{MftAttributeContent, MftAttributeType},
    MftEntry,
};

#[derive(Debug, Default)]
pub struct UndeleteFileEntry {
    pub filename: String,
    pub is_allocated: bool,
    pub record_number: u64,
    pub data: Vec<u8>,
}

impl From<MftEntry> for UndeleteFileEntry {
    fn from(value: MftEntry) -> Self {
        let file_attribute = value
            .iter_attributes_matching(Some(vec![MftAttributeType::FileName]))
            .filter_map(|attr| match attr {
                Ok(attribute) => match attribute.data {
                    MftAttributeContent::AttrX30(filename) => Some(filename),
                    _ => None,
                },
                Err(_) => None,
            })
            .next();

        let data = value
            .iter_attributes_matching(Some(vec![MftAttributeType::DATA]))
            .filter_map(|attr| match attr {
                Ok(attribute) => match attribute.data {
                    MftAttributeContent::AttrX80(data) => Some(data),
                    _ => None,
                },
                Err(_) => None,
            })
            .next();

        UndeleteFileEntry {
            filename: match file_attribute {
                Some(attr) => attr.name,
                None => "".to_string(),
            },
            is_allocated: value.is_allocated(),
            record_number: value.header.record_number,
            data: match data {
                Some(d) => d.data().to_vec(),
                None => vec![],
            },
        }
    }
}

impl Display for UndeleteFileEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.filename))
    }
}
