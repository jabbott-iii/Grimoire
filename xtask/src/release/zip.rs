//! A minimal zip writer and reader for the archives `cargo xtask release` makes
//! itself: Windows release archives and Python wheels (ADR-0009).
//!
//! Only what those need: ASCII names, deflate or stored members, no zip64, no
//! encryption, one disk, a fixed 1980-01-01 timestamp and Unix modes in the external
//! attributes (which pip and `unzip` honour, and Windows ignores). The `zip` crate
//! was left out because it brings many optional codecs and releases very often.

use flate2::Compression;
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use std::io::{Read as _, Write as _};

/// One member of a zip archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Member {
    pub(crate) name: String,
    /// Unix permission bits (for example `0o755`).
    pub(crate) mode: u32,
    pub(crate) data: Vec<u8>,
}

const LOCAL: u32 = 0x0403_4b50;
const CENTRAL: u32 = 0x0201_4b50;
const END: u32 = 0x0605_4b50;
/// MS-DOS date for 1980-01-01 (the earliest a zip can express); the time is 00:00.
const DOS_DATE: u16 = 0x0021;
const DEFLATE: u16 = 8;
const STORED: u16 = 0;
/// "Version made by": Unix (3), so the external attributes are a Unix mode; spec 2.0.
const MADE_BY: u16 = 0x0314;
const NEEDED: u16 = 20;
/// The most [`read`] inflates in all: release archives are a few megabytes.
pub(crate) const MAX_TOTAL: u64 = 256 << 20;
const MAX_MEMBERS: usize = 4096;

fn u16le(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn u32le(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn checked_u32(n: usize, what: &str) -> Result<u32, String> {
    u32::try_from(n).map_err(|_| format!("{what} is too large for a zip without zip64"))
}

/// Writes `members` in the order given (wheels need their `.dist-info` last).
pub(crate) fn write(members: &[Member]) -> Result<Vec<u8>, String> {
    if members.len() > usize::from(u16::MAX) {
        return Err("too many zip members".to_owned());
    }
    let mut out = Vec::new();
    let mut central = Vec::new();
    for m in members {
        if m.name.is_empty()
            || !m.name.is_ascii()
            || m.name.starts_with('/')
            || m.name.contains('\\')
            || m.name
                .split('/')
                .any(|p| p.is_empty() || p == "." || p == "..")
        {
            return Err(format!("unsafe zip member name {:?}", m.name));
        }
        let mut crc = flate2::Crc::new();
        crc.update(&m.data);
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
        encoder
            .write_all(&m.data)
            .and_then(|()| encoder.try_finish())
            .map_err(|e| format!("cannot compress {}: {e}", m.name))?;
        let compressed = encoder
            .finish()
            .map_err(|e| format!("cannot compress {}: {e}", m.name))?;
        let offset = checked_u32(out.len(), "the archive")?;
        let name_len = u16::try_from(m.name.len()).map_err(|_| "zip member name too long")?;
        let csize = checked_u32(compressed.len(), &m.name)?;
        let usize_ = checked_u32(m.data.len(), &m.name)?;

        u32le(&mut out, LOCAL);
        u16le(&mut out, NEEDED);
        u16le(&mut out, 0); // flags
        u16le(&mut out, DEFLATE);
        u16le(&mut out, 0); // time
        u16le(&mut out, DOS_DATE);
        u32le(&mut out, crc.sum());
        u32le(&mut out, csize);
        u32le(&mut out, usize_);
        u16le(&mut out, name_len);
        u16le(&mut out, 0); // extra
        out.extend_from_slice(m.name.as_bytes());
        out.extend_from_slice(&compressed);

        u32le(&mut central, CENTRAL);
        u16le(&mut central, MADE_BY);
        u16le(&mut central, NEEDED);
        u16le(&mut central, 0);
        u16le(&mut central, DEFLATE);
        u16le(&mut central, 0);
        u16le(&mut central, DOS_DATE);
        u32le(&mut central, crc.sum());
        u32le(&mut central, csize);
        u32le(&mut central, usize_);
        u16le(&mut central, name_len);
        u16le(&mut central, 0); // extra
        u16le(&mut central, 0); // comment
        u16le(&mut central, 0); // disk
        u16le(&mut central, 0); // internal attributes
        // A regular file (S_IFREG) with the given permissions, in the high 16 bits.
        u32le(&mut central, (0o100_000 | (m.mode & 0o7777)) << 16);
        u32le(&mut central, offset);
        central.extend_from_slice(m.name.as_bytes());
    }
    let count = u16::try_from(members.len()).map_err(|_| "too many zip members")?;
    let cd_offset = checked_u32(out.len(), "the archive")?;
    let cd_size = checked_u32(central.len(), "the central directory")?;
    out.extend_from_slice(&central);
    u32le(&mut out, END);
    u16le(&mut out, 0);
    u16le(&mut out, 0);
    u16le(&mut out, count);
    u16le(&mut out, count);
    u32le(&mut out, cd_size);
    u32le(&mut out, cd_offset);
    u16le(&mut out, 0); // comment
    Ok(out)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl Cursor<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], String> {
        let end = self
            .pos
            .checked_add(n)
            .filter(|end| *end <= self.bytes.len())
            .ok_or("truncated zip")?;
        let slice = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn u16(&mut self) -> Result<u16, String> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    fn u32(&mut self) -> Result<u32, String> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
}

/// One central-directory record.
struct Entry {
    name: String,
    flags: u16,
    method: u16,
    crc: u32,
    csize: usize,
    usize: usize,
    external: u32,
    local: usize,
}

/// The central directory's records and its offset.
fn central_directory(bytes: &[u8]) -> Result<(Vec<Entry>, usize), String> {
    // The end record is the last 22 bytes when there is no archive comment, which is
    // all this module writes; search back further for archives that have one.
    let min = bytes.len().saturating_sub(22 + usize::from(u16::MAX));
    let end = (min..=bytes.len().saturating_sub(22))
        .rev()
        .find(|&i| bytes[i..].starts_with(&END.to_le_bytes()))
        .ok_or("not a zip archive (no end of central directory record)")?;
    let mut c = Cursor {
        bytes,
        pos: end + 4,
    };
    let (disk, cd_disk) = (c.u16()?, c.u16()?);
    let (_, count) = (c.u16()?, usize::from(c.u16()?));
    let (_, cd_offset) = (c.u32()?, c.u32()?);
    if disk != 0 || cd_disk != 0 || count > MAX_MEMBERS {
        return Err("unsupported zip (multi-disk or too many members)".to_owned());
    }
    let cd_offset = usize::try_from(cd_offset).map_err(|_| "bad zip offset")?;
    let mut c = Cursor {
        bytes,
        pos: cd_offset,
    };
    let mut entries = Vec::with_capacity(count);
    let mut total: u64 = 0;
    for _ in 0..count {
        if c.u32()? != CENTRAL {
            return Err("corrupt zip central directory".to_owned());
        }
        c.take(4)?; // version made by, version needed
        let flags = c.u16()?;
        let method = c.u16()?;
        c.take(4)?; // time, date
        let crc = c.u32()?;
        let csize = usize::try_from(c.u32()?).map_err(|_| "bad zip size")?;
        let declared = c.u32()?;
        let name_len = usize::from(c.u16()?);
        let extra_len = usize::from(c.u16()?);
        let comment_len = usize::from(c.u16()?);
        c.take(4)?; // disk, internal attributes
        let external = c.u32()?;
        let local = usize::try_from(c.u32()?).map_err(|_| "bad zip offset")?;
        let name = std::str::from_utf8(c.take(name_len)?)
            .map_err(|_| "non-UTF-8 zip member name")?
            .to_owned();
        c.take(extra_len + comment_len)?;
        total += u64::from(declared);
        if total > MAX_TOTAL {
            return Err(format!("the zip holds more than {MAX_TOTAL} bytes"));
        }
        entries.push(Entry {
            name,
            flags,
            method,
            crc,
            csize,
            usize: usize::try_from(declared).map_err(|_| "bad zip size")?,
            external,
            local,
        });
    }
    Ok((entries, cd_offset))
}

/// Refuses members that overlap each other or the central directory, and duplicate
/// names, so every byte of data belongs to exactly one member.
fn check_layout(bytes: &[u8], entries: &[Entry], cd_offset: usize) -> Result<(), String> {
    // Every member's [header, end of data) range: they must not overlap each other or
    // the central directory, and no name may appear twice.
    let mut ranges = Vec::with_capacity(entries.len());
    let mut names: Vec<&str> = Vec::with_capacity(entries.len());
    for e in entries {
        let mut l = Cursor {
            bytes,
            pos: e.local,
        };
        if l.u32()? != LOCAL {
            return Err(format!("{}: corrupt zip local header", e.name));
        }
        l.take(22)?;
        let skip = usize::from(l.u16()?) + usize::from(l.u16()?);
        let data_end = l
            .pos
            .checked_add(skip)
            .and_then(|p| p.checked_add(e.csize))
            .ok_or("bad zip size")?;
        if data_end > cd_offset {
            return Err(format!(
                "{}: zip member data runs into the central directory",
                e.name
            ));
        }
        ranges.push((e.local, data_end));
        names.push(&e.name);
    }
    ranges.sort_unstable();
    if ranges.windows(2).any(|w| w[1].0 < w[0].1) {
        return Err("zip members overlap".to_owned());
    }
    names.sort_unstable();
    if names.windows(2).any(|w| w[0] == w[1]) {
        return Err("duplicate zip member name".to_owned());
    }
    Ok(())
}

/// Reads every member of a zip archive, checking sizes and CRCs. Archives are hostile
/// input: duplicate names, members that overlap or share data, and more than
/// [`MAX_TOTAL`] bytes in all are refused before anything is inflated.
pub(crate) fn read(bytes: &[u8]) -> Result<Vec<Member>, String> {
    let (entries, cd_offset) = central_directory(bytes)?;
    check_layout(bytes, &entries, cd_offset)?;
    let mut members = Vec::with_capacity(entries.len());
    for e in entries {
        if e.flags & 0x0001 != 0 {
            return Err(format!(
                "{}: encrypted zip members are not supported",
                e.name
            ));
        }
        let mut l = Cursor {
            bytes,
            pos: e.local + 26,
        };
        let skip = usize::from(l.u16()?) + usize::from(l.u16()?);
        l.take(skip)?;
        let raw = l.take(e.csize)?;
        let data = match e.method {
            STORED => raw.to_vec(),
            DEFLATE => {
                let mut data = Vec::with_capacity(e.usize);
                DeflateDecoder::new(raw)
                    .take(e.usize as u64 + 1)
                    .read_to_end(&mut data)
                    .map_err(|err| format!("{}: cannot inflate: {err}", e.name))?;
                data
            }
            other => {
                return Err(format!(
                    "{}: unsupported compression method {other}",
                    e.name
                ));
            }
        };
        let mut check = flate2::Crc::new();
        check.update(&data);
        if data.len() != e.usize || check.sum() != e.crc {
            return Err(format!("{}: size or CRC mismatch", e.name));
        }
        // Unix permission bits; archives made on Windows leave them zero.
        let mode = (e.external >> 16) & 0o7777;
        members.push(Member {
            name: e.name,
            mode,
            data,
        });
    }
    Ok(members)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_keeps_order_modes_and_bytes() {
        let members = vec![
            Member {
                name: "grimoire.exe".to_owned(),
                mode: 0o755,
                data: (0..50_000u32).map(|i| (i % 251) as u8).collect(),
            },
            Member {
                name: "dir/empty".to_owned(),
                mode: 0o644,
                data: Vec::new(),
            },
        ];
        let bytes = write(&members).unwrap();
        assert_eq!(read(&bytes).unwrap(), members);
        assert_eq!(write(&members).unwrap(), bytes, "deterministic");
    }

    #[test]
    fn rejects_unsafe_names_and_corruption() {
        for name in ["", "/abs", "a/../b", "a\\b", "a//b", "caf\u{e9}"] {
            let m = Member {
                name: name.to_owned(),
                mode: 0o644,
                data: vec![1],
            };
            assert!(write(&[m]).is_err(), "{name:?}");
        }
        let mut bytes = write(&[Member {
            name: "a".to_owned(),
            mode: 0o644,
            data: b"hello hello hello".to_vec(),
        }])
        .unwrap();
        // Flip a byte inside the compressed data (after the 31-byte local header).
        bytes[32] ^= 0xff;
        assert!(read(&bytes).is_err());
        assert!(read(b"not a zip").is_err());
    }

    #[test]
    fn rejects_duplicate_names_and_shared_data() {
        let one = |name: &str| Member {
            name: name.to_owned(),
            mode: 0o644,
            data: b"payload".to_vec(),
        };
        // A writer that allowed it: two central-directory entries with one name.
        let dup = write(&[one("a"), one("b")]).unwrap();
        let mut renamed = dup.clone();
        let at = renamed.len() - 22 - 1; // the last central record's one-byte name
        assert_eq!(renamed[at], b'b');
        renamed[at] = b'a';
        assert!(read(&renamed).unwrap_err().contains("duplicate"));
        // Two central-directory entries that point at the same local header.
        let mut shared = dup;
        let cd = usize::try_from(u32::from_le_bytes(
            shared[shared.len() - 6..shared.len() - 2]
                .try_into()
                .unwrap(),
        ))
        .unwrap();
        let second = cd + 47; // the second central record
        let offset_at = second + 42;
        shared[offset_at..offset_at + 4].copy_from_slice(&0u32.to_le_bytes());
        assert!(read(&shared).is_err());
    }
}
