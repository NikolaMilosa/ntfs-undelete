use mft::{
    attribute::{
        non_resident_attr::NonResidentAttr, x80::DataAttr, MftAttributeContent, MftAttributeType,
    },
    MftEntry,
};

use crate::errors::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
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

const CLUSTER_SIZE: usize = 4096; // 4 KiB
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
        let offset = find_mft_signature(self.path.as_path())?;

        let mut file = File::open(&self.path)?;
        let mut mft_entry = vec![0; ENTRY_SIZE];
        file.seek(SeekFrom::Start(offset as u64))?; // Seek to the start of the MFT
        file.read_exact(&mut mft_entry)?; // Read the first entry

        let entry = MftEntry::from_buffer(mft_entry, 0)?;

        self.read_data_from_entry(entry)
    }

    pub fn read_data_from_entry(&self, entry: MftEntry) -> Result<Vec<u8>> {
        let attribute = match entry
            .iter_attributes_matching(Some(vec![MftAttributeType::DATA]))
            .filter_map(|a| a.ok())
            .next()
        {
            Some(attr) => attr,
            None => {
                return Err(crate::errors::Error::Any {
                    detail: "Couldn't find non-residental header".to_string(),
                })
            }
        };

        match attribute.data {
            MftAttributeContent::AttrX80(data) => self.read_from_attr_80(data),
            MftAttributeContent::DataRun(data) => self.read_from_data_run(data),
            _ => Err(crate::errors::Error::Any {
                detail: "Couldn't read data from attribute".to_string(),
            }),
        }
    }

    fn read_from_attr_80(&self, data: DataAttr) -> Result<Vec<u8>> {
        Ok(data.data().to_vec())
    }

    fn read_from_data_run(&self, data: NonResidentAttr) -> Result<Vec<u8>> {
        let mut file = match self.reader_type {
            ReaderType::Directory => {
                let block_device = find_block_device(&self.path)?;
                let block_device = block_device.ok_or_else(|| crate::errors::Error::Any {
                    detail: "Couldn't find block device for directory".to_string(),
                })?;

                File::open(block_device)?
            }
            _ => File::open(&self.path)?,
        };
        let mut bytes = vec![];
        for dr in data.data_runs {
            let mut cluster = vec![0; dr.lcn_length as usize * CLUSTER_SIZE];
            file.seek(SeekFrom::Start(dr.lcn_offset * CLUSTER_SIZE as u64))?;
            file.read_exact(&mut cluster)?;
            bytes.extend(cluster);
        }

        Ok(bytes)
    }
}

fn find_mft_signature<P>(path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut total_length = file.metadata()?.len() as i64;
    let mut buffer_size = 4 * CLUSTER_SIZE;

    loop {
        total_length -= buffer_size as i64;
        if total_length < 0 {
            buffer_size += buffer_size.wrapping_add(total_length as usize) // for the last entry section
        }
        let mut buffer = vec![0; buffer_size];
        file.read_exact(&mut buffer)?;
        let found = (0..buffer.len() - 4).find(|&i| SIGNATURES.contains(&&buffer[i..i + 4]));
        if let Some(offset) = found {
            return Ok(offset);
        }
        if total_length < 0 {
            return Err(crate::errors::Error::Any {
                detail: "read the whole disk, couldn't find signature".to_string(),
            });
        }
    }
}

fn find_block_device(mount_point: &Path) -> std::io::Result<Option<PathBuf>> {
    let mounts_file = File::open("/proc/mounts")?;
    let reader = BufReader::new(mounts_file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            let mount_path = Path::new(parts[1]);
            let block_device = Path::new(parts[0]);

            if mount_path == mount_point {
                return Ok(Some(block_device.to_path_buf()));
            }
        }
    }

    Ok(None)
}
