use mft::{
    attribute::{header::ResidentialHeader, MftAttributeType},
    MftEntry,
};

use crate::errors::Result;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Reader {
    path: PathBuf,
    reader_type: ReaderType,
}

#[derive(Debug)]
enum ReaderType {
    Directory,
    Image,
    BlockDevice,
}

const SECTOR_SIZE: usize = 512; // 512 Bytes
const CLUSTER_SIZE: usize = 32 * 2 * SECTOR_SIZE; // 32 KiB
const ENTRY_SIZE: usize = 1024; // 1 KiB
const SIGNATURES: [&[u8]; 3] = [b"FILE", b"BAAD", b"0000"];

impl Reader {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let reader_type = if path.is_dir() {
            ReaderType::Directory
        } else if path.is_file() {
            ReaderType::Image
        } else if path.starts_with("/dev/") {
            ReaderType::BlockDevice
        } else {
            return Err(crate::errors::Error::FailedToOpenFile {
                path,
                source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
            });
        };

        Ok(Self { path, reader_type })
    }

    pub fn read_mft(&self) -> Result<Vec<u8>> {
        match self.reader_type {
            ReaderType::Directory => self.read_mft_dir(),
            ReaderType::Image => self.read_mft_bytes(),
            ReaderType::BlockDevice => self.read_mft_bytes(),
        }
    }

    fn read_mft_dir(&self) -> Result<Vec<u8>> {
        let path = self.path.join("$MFT");

        if !path.exists() {
            return Err(crate::errors::Error::Any {
                detail: format!(
                    "Couldn't find $MFT in directory {}, select root directory of partition",
                    self.path.to_str().unwrap()
                ),
            });
        }

        let mut buffer = vec![];
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn read_mft_bytes(&self) -> Result<Vec<u8>> {
        let mut file = File::open(&self.path)?;

        let mut buffer = vec![0; SECTOR_SIZE * 20];
        file.read_exact(&mut buffer)?;

        let offset = find_mft_signature(&buffer).ok_or_else(|| crate::errors::Error::Any {
            detail: "Couldn't find MFT signature meaning that this is probably not NTFS partition"
                .to_string(),
        })?;

        let mut mft_entry = vec![0; ENTRY_SIZE];
        file.seek(SeekFrom::Start(offset as u64))?; // Seek to the start of the MFT
        file.read_exact(&mut mft_entry)?; // Read the first entry

        let entry = MftEntry::from_buffer(mft_entry, 0)?;

        let residental_header = match entry
            .iter_attributes()
            .filter_map(|a| a.ok())
            .filter(|a| a.header.type_code == MftAttributeType::DATA)
            .filter_map(|a| match a.header.residential_header {
                ResidentialHeader::Resident(_) => None,
                ResidentialHeader::NonResident(_) => Some(a),
            })
            .map(|a| a.header.residential_header)
            .next()
        {
            Some(ResidentialHeader::Resident(header)) => {
                return Err(crate::errors::Error::Any {
                    detail: format!(
                        "Residental header is resident which shouldn't happen: {:?}",
                        header
                    ),
                })
            }
            Some(ResidentialHeader::NonResident(header)) => header,
            None => {
                return Err(crate::errors::Error::Any {
                    detail: "Couldn't find non-residental header".to_string(),
                })
            }
        };

        let mut mft_bytes = vec![0; residental_header.allocated_length as usize];
        file.seek(SeekFrom::Start(
            offset as u64 + residental_header.vnc_first as u64 * CLUSTER_SIZE as u64,
        ))?;
        file.read_exact(&mut mft_bytes)?;

        Ok(mft_bytes)
    }
}

fn find_mft_signature(buffer: &[u8]) -> Option<usize> {
    for i in 0..buffer.len() - 4 {
        if SIGNATURES.contains(&&buffer[i..i + 4]) {
            return Some(i);
        }
    }

    None
}
