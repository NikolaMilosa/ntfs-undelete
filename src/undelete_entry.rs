use mft::{
    attribute::{MftAttributeContent, MftAttributeType},
    MftEntry,
};

#[derive(Debug, Default)]
pub struct UndeleteFileEntry {
    pub filename: String,
    pub is_allocated: bool,
    pub record_number: u64,
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

        UndeleteFileEntry {
            filename: match file_attribute {
                Some(attr) => attr.name,
                None => "".to_string(),
            },
            is_allocated: value.is_allocated(),
            record_number: value.header.record_number,
        }
    }
}
