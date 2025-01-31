//! [![crates.io](https://img.shields.io/crates/v/partymix)](https://crates.io/crates/partymix)
//! [![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
//! [![tests](https://github.com/pyx-cvm/partymix/actions/workflows/cargo-test.yml/badge.svg)](https://github.com/pyx-cvm/partymix/actions/workflows/cargo-test.yml)
//!
//! A tool for combining filesystem images into a disk image with a Master Boot Record (MBR).
//!
//! # Usage
//!
//! ```bash
//! $ partymix OUTPUT [[+]TYPE=IMAGE ...]
//! ```
//!
//! * The `+` sign indicates whether the partition is active or not.
//! * TYPE is either the hex value of the partition type or one of the supported aliases.
//! * IMAGE is the path to the filesystem image.
//!
//! # Supported Partition Aliases
//!
//! The following partition type aliases are supported:
//!
//! - fat12 (0x01)
//! - fat16 (0x04)
//! - ntfs (0x07)
//! - fat32 (0x0B)
//! - linuxswap (0x82)
//! - linux (0x83)
//! - efi (0xEF)
//!
//! # Example
//!
//! The following command creates a disk image with a FAT32 boot partition and
//! a Linux root partition:
//!
//! ```bash
//! $ partymix disk.img +fat32=boot.img linux=root.img
//! ```

#![forbid(clippy::expect_used, clippy::panic)]
#![deny(
    clippy::all,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    single_use_lifetimes,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_code,
    unreachable_patterns,
    unsafe_code,
    unstable_features,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

mod cli;
mod mbr;

use cli::{Error, Partition};

use std::fs::File;
use std::io::{Seek, Write};
use std::path::Path;

fn usage<'a, P: AsRef<Path>, M: Into<Option<&'a str>>>(exec: P, msg: M) -> ! {
    let exec = exec.as_ref().file_name().unwrap();

    eprintln!("Combines filesystem images into a partition image file with a MBR.");
    eprintln!("");
    eprintln!("$ {} OUTPUT [[+]TYPE=IMAGE ...]", exec.to_str().unwrap());
    eprintln!("");
    eprintln!("The `+` sign indicates whether the partition is active or not.");
    eprintln!("");
    eprintln!("TYPE can be either the hex value of the partition type or");
    eprintln!("one of the following aliases:");
    eprintln!("");

    for (alias, kind) in cli::Kind::ALIASES.iter() {
        eprintln!("  {:<16} {:02X}", alias, kind);
    }

    eprintln!("");

    if let Some(msg) = msg.into() {
        eprintln!("Error: {}", msg);
    }

    std::process::exit(1);
}

fn main() -> Result<(), Error> {
    // Get the arguments.
    let mut args = std::env::args();
    let exec = args.next().unwrap();
    let args = args.collect::<Vec<_>>();

    // Look for help.
    if args.iter().find(|a| *a == "--help").is_some() {
        usage(&exec, None);
    }

    // Parse the arguments.
    let mut args = args.into_iter();
    let path = match args.next() {
        Some(path) => path,
        None => usage(&exec, "missing output file"),
    };

    // Allocate space for all partitions in the table.
    let mut mbr = mbr::MasterBootRecord::default();
    let sources = args
        .into_iter()
        .map(|arg| {
            let part: Partition = arg.parse()?;
            mbr.add(part.active, *part.kind, part.size)
                .map(|o| (o, part.file, part.size))
                .ok_or_else(|| Error::from(format!("partition does not fit: {:?}", part.path)))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Create the output file and write the MBR.
    let mut dst = File::create_new(path)?;
    dst.write_all(mbr.as_ref())?;

    // Write each partition.
    for (offset, mut src, size) in sources {
        let _ = dst.seek(std::io::SeekFrom::Start(offset))?;
        let n = std::io::copy(&mut src, &mut dst)?;
        assert_eq!(n, size.get());
    }

    Ok(())
}
