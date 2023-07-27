use std::{fs::File, io::BufReader, path::PathBuf};

use log::{error, info, warn};
use mft::{attribute::MftAttributeContent, MftParser};

use crate::{errors::UndeleteError, undelete_entry::UndeleteFileEntry};

pub struct ParserWrapper {
    parser: MftParser<BufReader<File>>,
}

impl ParserWrapper {
    pub fn new(path: PathBuf) -> Result<Self, UndeleteError> {
        let parser = match MftParser::from_path(path) {
            Ok(p) => p,
            Err(e) => return Err(UndeleteError::InitializationError(e.to_string())),
        };
        Ok(Self { parser })
    }

    pub fn _print_all_attributes(&mut self) {
        for entry in self.parser.iter_entries() {
            match entry {
                Ok(e) => {
                    for attribute in e.iter_attributes().filter_map(|attr| attr.ok()) {
                        match attribute.data {
                            MftAttributeContent::AttrX10(standard_info) => {
                                info!(
                                    "Entry: {} - X10 attribute: {:?}",
                                    e.header.record_number, standard_info
                                )
                            }
                            MftAttributeContent::AttrX20(attribute_list_info) => {
                                info!(
                                    "Entry: {} - X20 attribute: {:?}",
                                    e.header.record_number, attribute_list_info
                                )
                            }
                            MftAttributeContent::AttrX30(filename) => {
                                info!(
                                    "Entry: {} - X30 attribute: {:?}",
                                    e.header.record_number, filename
                                )
                            }
                            MftAttributeContent::AttrX40(object_id_info) => {
                                info!(
                                    "Entry: {} - X40 attribute: {:?}",
                                    e.header.record_number, object_id_info
                                )
                            }
                            MftAttributeContent::AttrX80(data_info) => {
                                info!(
                                    "Entry: {} - X80 attribute: {:?}",
                                    e.header.record_number, data_info
                                )
                            }
                            MftAttributeContent::AttrX90(index_root_info) => {
                                info!(
                                    "Entry: {} - X90 attribute: {:?}",
                                    e.header.record_number, index_root_info
                                )
                            }
                            MftAttributeContent::Raw(data) => {
                                info!(
                                    "Entry: {} - Raw attribute: {:?}",
                                    e.header.record_number, data
                                )
                            }
                            _ => {
                                info!(
                                    "Entry: {} - Other attribute: {:?}",
                                    e.header.record_number, attribute
                                )
                            }
                        }
                    }
                }
                Err(e) => error!("Error parsing entry: {}", e),
            }
        }
    }

    pub fn get_all_entry_names(&mut self) -> Vec<UndeleteFileEntry> {
        let found = self
            .parser
            .iter_entries()
            .filter_map(|e| match e {
                Ok(entry) => Some(entry.into()),
                Err(_) => None,
            })
            .filter(|e: &UndeleteFileEntry| !e.filename.is_empty())
            .collect::<Vec<_>>();

        let found_entries = self.parser.iter_entries().count();
        if found.len() != found_entries {
            warn!(
                "Some entries were not parsed correctly. Found: {}, Parsed: {}",
                found.len(),
                found_entries
            );
        }

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_created_correctly() {
        ParserWrapper::new(PathBuf::from("test_data/MFT_TEST")).unwrap();
    }

    #[test]
    #[should_panic]
    fn parser_created_incorrectly() {
        ParserWrapper::new(PathBuf::from("test_data/MFT_TEST_NOT_EXISTING")).unwrap();
    }

    #[test]
    fn parser_get_entry_names() {
        let entries = ParserWrapper::new(PathBuf::from("test_data/MFT_TEST"))
            .unwrap()
            .get_all_entry_names();

        assert!(!entries.is_empty());
    }
}
