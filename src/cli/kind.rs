use std::num::{NonZeroU8, ParseIntError};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Kind(NonZeroU8);

impl Kind {
    pub const ALIASES: &'static [(&'static str, u8)] = &[
        ("fat12", 0x01),
        ("fat16", 0x04),
        ("ntfs", 0x07),
        ("fat32", 0x0C),
        ("linuxswap", 0x82),
        ("linux", 0x83),
        ("efi", 0xEF),
    ];
}

impl Deref for Kind {
    type Target = NonZeroU8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Kind {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for (name, number) in Self::ALIASES {
            if s == *name {
                return Ok(Self(NonZeroU8::new(*number).unwrap()));
            }
        }

        let error = u8::from_str_radix("Z", 16).unwrap_err();
        let n = u8::from_str_radix(&s, 16)?;
        let n = NonZeroU8::new(n).ok_or(error)?;
        Ok(Self(n))
    }
}
