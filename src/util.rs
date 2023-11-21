use crate::errors::Result;
use std::{
    fmt::Display,
    fs::File,
    io::{Read, Seek},
    path::Path,
};
#[derive(PartialEq, Clone)]
pub enum FileSystems {
    Ntfs(NtfsBootSector),
    Fat,
    Ext,
    Iso9660,
    Hfs,
    Unknown,
}

impl From<FileSystems> for &str {
    fn from(value: FileSystems) -> Self {
        match value {
            FileSystems::Ntfs(_) => "Ntfs",
            FileSystems::Fat => "Fat",
            FileSystems::Ext => "Ext",
            FileSystems::Iso9660 => "Iso9660",
            FileSystems::Hfs => "Hfs",
            FileSystems::Unknown => "Unknown",
        }
    }
}

impl Display for FileSystems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.clone().into())
    }
}

pub fn detect_file_system<P>(path: P) -> Result<FileSystems>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut boot_sector = [0u8; 512];

    file.read_exact(&mut boot_sector)?;

    if &boot_sector[3..7] == b"Ntfs" {
        return Ok(FileSystems::Ntfs(NtfsBootSector::from_bytes(&boot_sector)?));
    }

    if &boot_sector[36..39] == b"Fat" {
        return Ok(FileSystems::Fat);
    }

    if &boot_sector[0..2] == b"H+" || &boot_sector[0..4] == b"HfsJ" || &boot_sector[0..4] == b"Hfs+"
    {
        return Ok(FileSystems::Hfs);
    }

    if &boot_sector[56..58] == b"\x53\xEF" {
        return Ok(FileSystems::Ext);
    }

    let mut iso_buffer = [0u8; 5];
    file.seek(std::io::SeekFrom::Start(32769))?;
    file.read_exact(&mut iso_buffer)?;

    if &iso_buffer[..] == b"CD001" {
        return Ok(FileSystems::Iso9660);
    }

    Ok(FileSystems::Unknown)
}

#[derive(Debug, Clone, PartialEq)]
pub struct NtfsBootSector {
    oem_id: String,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    media_descriptor: u8,
    sectors_per_track: u16,
    num_heads: u16,
    hidden_sectors: u32,
    total_sectors: u64,
    logical_cluster_mft: u64,
    logical_cluster_mft_mirror: u64,
    clusters_per_file_record_segment: u32,
    clusters_per_index_buffer: u8,
    volume_serial_number: u64,
    checksum: u32,
}

impl NtfsBootSector {
    fn from_bytes(boot_sector_data: &[u8]) -> Result<Self> {
        Ok(Self {
            oem_id: String::from_utf8_lossy(&boot_sector_data[3..11]).to_string(),
            bytes_per_sector: u16::from_le_bytes([boot_sector_data[0x0B], boot_sector_data[0x0C]]),
            sectors_per_cluster: boot_sector_data[0x0D],
            reserved_sectors: u16::from_le_bytes([boot_sector_data[0x0E], boot_sector_data[0x0F]]),
            media_descriptor: boot_sector_data[0x15],
            sectors_per_track: u16::from_le_bytes([boot_sector_data[0x18], boot_sector_data[0x19]]),
            num_heads: u16::from_le_bytes([boot_sector_data[0x1A], boot_sector_data[0x1B]]),
            hidden_sectors: u32::from_le_bytes([
                boot_sector_data[0x1C],
                boot_sector_data[0x1D],
                boot_sector_data[0x1E],
                boot_sector_data[0x1F],
            ]),
            total_sectors: u64::from_le_bytes([
                boot_sector_data[0x28],
                boot_sector_data[0x29],
                boot_sector_data[0x2A],
                boot_sector_data[0x2B],
                boot_sector_data[0x2C],
                boot_sector_data[0x2D],
                boot_sector_data[0x2E],
                boot_sector_data[0x2F],
            ]),
            logical_cluster_mft: u64::from_le_bytes([
                boot_sector_data[0x30],
                boot_sector_data[0x31],
                boot_sector_data[0x32],
                boot_sector_data[0x33],
                boot_sector_data[0x34],
                boot_sector_data[0x35],
                boot_sector_data[0x36],
                boot_sector_data[0x37],
            ]),
            logical_cluster_mft_mirror: u64::from_le_bytes([
                boot_sector_data[0x38],
                boot_sector_data[0x39],
                boot_sector_data[0x3A],
                boot_sector_data[0x3B],
                boot_sector_data[0x3C],
                boot_sector_data[0x3D],
                boot_sector_data[0x3E],
                boot_sector_data[0x3F],
            ]),
            clusters_per_file_record_segment: u32::from_le_bytes([
                boot_sector_data[0x40],
                boot_sector_data[0x41],
                boot_sector_data[0x42],
                boot_sector_data[0x43],
            ]),
            clusters_per_index_buffer: boot_sector_data[0x44],
            volume_serial_number: u64::from_le_bytes([
                boot_sector_data[0x48],
                boot_sector_data[0x49],
                boot_sector_data[0x4A],
                boot_sector_data[0x4B],
                boot_sector_data[0x4C],
                boot_sector_data[0x4D],
                boot_sector_data[0x4E],
                boot_sector_data[0x4F],
            ]),
            checksum: u32::from_le_bytes([
                boot_sector_data[0x50],
                boot_sector_data[0x51],
                boot_sector_data[0x52],
                boot_sector_data[0x53],
            ]),
        })
    }
}
