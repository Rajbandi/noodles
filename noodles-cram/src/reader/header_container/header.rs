use std::io::{self, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::CrcReader;

use crate::reader::num::{read_itf8, read_ltf8};

pub(super) fn read_header<R>(reader: &mut R) -> io::Result<usize>
where
    R: Read,
{
    let mut crc_reader = CrcReader::new(reader);

    let length = crc_reader.read_i32::<LittleEndian>().and_then(|n| {
        usize::try_from(n).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    })?;

    // reference sequence ID
    read_itf8(&mut crc_reader)?;

    // alignment start
    read_itf8(&mut crc_reader)?;

    // alignment span
    read_itf8(&mut crc_reader)?;

    // record count
    read_itf8(&mut crc_reader)?;

    // record counter
    read_ltf8(&mut crc_reader)?;

    // base count
    read_ltf8(&mut crc_reader)?;

    // block count
    read_itf8(&mut crc_reader).and_then(|n| {
        usize::try_from(n).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    })?;

    read_landmarks(&mut crc_reader)?;

    let actual_crc32 = crc_reader.crc().sum();

    let reader = crc_reader.into_inner();
    let expected_crc32 = reader.read_u32::<LittleEndian>()?;

    if actual_crc32 != expected_crc32 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "container header checksum mismatch: expected {expected_crc32:08x}, got {actual_crc32:08x}"
            ),
        ));
    }

    Ok(length)
}

fn read_landmarks<R>(reader: &mut R) -> io::Result<()>
where
    R: Read,
{
    let len = read_itf8(reader).and_then(|n| {
        usize::try_from(n).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    })?;

    for _ in 0..len {
        read_itf8(reader)?;
    }

    Ok(())
}
