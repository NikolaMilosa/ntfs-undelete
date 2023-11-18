# Rust NTFS Undelete Tool

![Build Status](https://img.shields.io/github/actions/workflow/status/NikolaMilosa/ntfs-undelete/tests.yml?branch=main)
![License](https://img.shields.io/github/license/NikolaMilosa/ntfs-undelete)
![Latest Release](https://img.shields.io/github/v/release/NikolaMilosa/ntfs-undelete)

## Description

The Rust NTFS Undelete Tool is a command-line utility for recovering deleted files from NTFS (New Technology File System) volumes. It leverages the power of Rust programming language to provide fast and reliable file recovery capabilities, minimizing the risk of further data loss during the process.

## Features

- Efficient and speedy file recovery from NTFS volumes.
- Supports the recovery of various file types, including documents, images, videos, and more.
- Recovers files while maintaining their original timestamps and attributes.
- Allows users to specify a target output directory for recovered files.
- Provides a dry run option for simulating the recovery process without actually writing files to the output directory.
- Supports recovering nested data
- Has built in file system detection which prevent running the tool on non-NTFS volumes.

### Prerequisites

- Rust programming language must be installed on your system. If not, you can download it from the official Rust website: https://www.rust-lang.org/tools/install


### Building from Source

1. Clone the repository:

    ```bash
    git clone https://github.com/NikolaMilosa/ntfs-undelete.git
    cd ntfs-undelete
    ```

2. Update submodules:
    
    ```bash
    git submodule update --init --recursive
    ```

3. Build the project with cargo:

    ```bash
    cargo build --release
    ```

4. The binary will be available in the `target/release/` directory. You can either add this directory to your `PATH` environment variable or copy the binary to your desired location.

### Download a prebuilt binary
*TODO!*

### Usage

```bash
(sudo) ntfs-undelete --output-dir <output_directory> --image <image> [--dry-run]
```

- `-i`,`--image`: 
    - The path to the NTFS image from which you want to recover deleted files. The image can be obtained with [`dd`](https://www.geeksforgeeks.org/dd-command-linux/)
    - The path to the `/dev/sdX` of the device from which you want to recover deleted files. 
- `-o`,`--output-dir`: The directory where recovered files will be stored. It must already exist.
- `-d`,`--dry-run`: Perform a dry run, simulating the recovery process without actually writing files to the output directory.

## Examples

1. Recover deleted files from the NTFS volume at `/dev/sda1` and save them to the `recovery_output` directory:

    ```bash
    sudo ntfs-undelete --output-dir recovery_output --image /dev/sda1 
    ```
2. Recover deleted files from the NTFS image at `ntfs_image.dd` and save them to the `recovery_output` directory:

    ```bash
    ntfs-undelete --output-dir recovery_output --image ntfs_image.dd
    ```
3. Recover deleted files from the mounted NTFS volume:

    ```bash
    # Usually automatically mounted by the OS
    # sudo mount /dev/sda1 /media/mnt
     
    ntfs-undelete --output-dir recovery_output --image /media/mnt
    ```
## Limitations

- The tool cannot recover files that have been overwritten since deletion.
- It is recommended to run the tool on a disk image to avoid potential data corruption on the original disk.
- The success of file recovery heavily depends on how much data has been overwritten since deletion, as well as the fragmentation of the file.

## Contributing

Contributions to the Rust NTFS Undelete Tool are welcome! If you find a bug or have an idea for improvement, please open an issue or submit a pull request following the contribution guidelines in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](LICENSE).
