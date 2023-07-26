use std::{fs::File, io::BufReader, path::PathBuf};

use log::{error, info};
use mft::{attribute::MftAttributeContent, MftParser};

use crate::errors::UndeleteError;

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

    pub fn print(&mut self) {
        for entry in self.parser.iter_entries() {
            match entry {
                Ok(e) => {
                    for attribute in e.iter_attributes().filter_map(|attr| attr.ok()) {
                        match attribute.data {
                            MftAttributeContent::AttrX10(standard_info) => {
                                info!("X10 attribute: {:?}", standard_info)
                            }
                            MftAttributeContent::AttrX20(attribute_list_info) => {
                                info!("X20 attribute: {:?}", attribute_list_info)
                            }
                            MftAttributeContent::AttrX30(filename) => {
                                info!("X30 attribute: {:?}", filename)
                            }
                            MftAttributeContent::AttrX40(object_id_info) => {
                                info!("X40 attribute: {:?}", object_id_info)
                            }
                            MftAttributeContent::AttrX80(data_info) => {
                                info!("X80 attribute: {:?}", data_info)
                            }
                            MftAttributeContent::AttrX90(index_root_info) => {
                                info!("X90 attribute: {:?}", index_root_info)
                            }
                            MftAttributeContent::Raw(data) => {
                                info!("Raw attribute: {:?}", data)
                            }
                            _ => {
                                info!("Other attribute: {:?}", attribute)
                            }
                        }
                    }
                }
                Err(e) => error!("Error parsing entry: {}", e),
            }
        }
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
}
