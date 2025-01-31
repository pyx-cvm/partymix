# partymix

[![crates.io](https://img.shields.io/crates/v/partymix)](https://crates.io/crates/partymix)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![tests](https://github.com/pyx-cvm/partymix/actions/workflows/cargo-test.yml/badge.svg)](https://github.com/pyx-cvm/partymix/actions/workflows/cargo-test.yml)

A tool for combining filesystem images into a disk image with a Master Boot Record (MBR).

## Usage

```bash
$ partymix OUTPUT [[+]TYPE=IMAGE ...]
```

* The `+` sign indicates whether the partition is active or not.
* TYPE is either the hex value of the partition type or one of the supported aliases.
* IMAGE is the path to the filesystem image.

## Supported Partition Aliases

The following partition type aliases are supported:

- fat12 (0x01)
- fat16 (0x04)
- ntfs (0x07)
- fat32 (0x0B)
- linuxswap (0x82)
- linux (0x83)
- efi (0xEF)

## Example

The following command creates a disk image with a FAT32 boot partition and
a Linux root partition:

```bash
$ partymix disk.img +fat32=boot.img linux=root.img
```

License: MIT
