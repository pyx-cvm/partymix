use std::mem::size_of_val;
use std::num::{NonZeroU64, NonZeroU8};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C, packed)]
struct Entry {
    status: u8, // 0x80 = bootable/active
    first_chs: [u8; 3],
    kind: u8,
    last_chs: [u8; 3],
    first_lba: [u8; 4],
    num_sectors: [u8; 4],
}

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct MasterBootRecord {
    code: [u8; 446],
    entries: [Entry; 4],
    signature: [u8; 2],
}

impl Default for MasterBootRecord {
    fn default() -> Self {
        Self {
            code: [0; 446],
            entries: [Entry::default(); 4],
            signature: [0x55, 0xAA],
        }
    }
}

impl AsRef<[u8]> for MasterBootRecord {
    #[allow(unsafe_code)]
    fn as_ref(&self) -> &[u8] {
        let ptr: *const Self = self;
        unsafe { std::slice::from_raw_parts(ptr as *const u8, size_of_val(self)) }
    }
}

impl MasterBootRecord {
    const SECTOR_SIZE: u64 = 512;

    /// Adds a partition to the table with the given properties.
    ///
    /// Returns the offset in bytes to the start of the partition data.
    pub fn add(&mut self, active: bool, kind: NonZeroU8, bytes: NonZeroU64) -> Option<u64> {
        let sectors = u32::try_from(bytes.get().div_ceil(Self::SECTOR_SIZE)).ok()?;

        let mut start: u32 = 1;
        for entry in self.entries.iter_mut() {
            if entry.kind == 0 {
                entry.num_sectors = sectors.to_le_bytes();
                entry.first_lba = start.to_le_bytes();
                entry.status = if active { 0x80 } else { 0x00 };
                entry.kind = kind.get();
                return Some(Self::SECTOR_SIZE * u64::from(start));
            } else {
                let first = u32::from_le_bytes(entry.first_lba);
                let count = u32::from_le_bytes(entry.num_sectors);
                start = first + count;
            }
        }

        None
    }
}

#[cfg(test)]
#[test]
fn entry() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Entry>(), 1);
    assert_eq!(size_of::<Entry>(), 16);
}

#[cfg(test)]
#[test]
fn mbr() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<MasterBootRecord>(), 1);
    assert_eq!(size_of::<MasterBootRecord>(), 512);

    let kind: crate::cli::Kind = "linux".parse().unwrap();

    let nz = |n: u64| NonZeroU64::new(n).unwrap();
    let mut mbr = MasterBootRecord::default();
    let a = mbr.add(true, *kind, nz(1)).unwrap();
    let b = mbr.add(false, *kind, nz(513)).unwrap();
    let c = mbr.add(true, *kind, nz(1024)).unwrap();
    let d = mbr.add(false, *kind, nz(1025)).unwrap();
    assert!(mbr.add(true, *kind, nz(1024)).is_none());

    assert_eq!(a, 512);
    assert_eq!(b, 1024);
    assert_eq!(c, 2048);
    assert_eq!(d, 3072);

    assert_eq!(mbr.entries[0].status, 0x80);
    assert_eq!(mbr.entries[0].kind, kind.get());
    assert_eq!(u32::from_le_bytes(mbr.entries[0].first_lba), 1);
    assert_eq!(u32::from_le_bytes(mbr.entries[0].num_sectors), 1);

    assert_eq!(mbr.entries[1].status, 0x00);
    assert_eq!(mbr.entries[1].kind, kind.get());
    assert_eq!(u32::from_le_bytes(mbr.entries[1].first_lba), 2);
    assert_eq!(u32::from_le_bytes(mbr.entries[1].num_sectors), 2);

    assert_eq!(mbr.entries[2].status, 0x80);
    assert_eq!(mbr.entries[2].kind, kind.get());
    assert_eq!(u32::from_le_bytes(mbr.entries[2].first_lba), 4);
    assert_eq!(u32::from_le_bytes(mbr.entries[2].num_sectors), 2);

    assert_eq!(mbr.entries[3].status, 0x00);
    assert_eq!(mbr.entries[3].kind, kind.get());
    assert_eq!(u32::from_le_bytes(mbr.entries[3].first_lba), 6);
    assert_eq!(u32::from_le_bytes(mbr.entries[3].num_sectors), 3);
}
