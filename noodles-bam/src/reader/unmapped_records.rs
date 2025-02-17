use std::io::{self, Read};

use noodles_bgzf as bgzf;
use noodles_sam::{self as sam, alignment::Record};

use super::Reader;

/// An iterator over unmapped records of a BAM reader.
///
/// This is created by calling [`Reader::query_unmapped`].
pub struct UnmappedRecords<'a, R>
where
    R: Read,
{
    reader: &'a mut Reader<bgzf::Reader<R>>,
    header: sam::Header,
    record: Record,
}

impl<'a, R> UnmappedRecords<'a, R>
where
    R: Read,
{
    pub(crate) fn new(reader: &'a mut Reader<bgzf::Reader<R>>) -> Self {
        Self {
            reader,
            header: sam::Header::default(),
            record: Record::default(),
        }
    }
}

impl<'a, R> Iterator for UnmappedRecords<'a, R>
where
    R: Read,
{
    type Item = io::Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.read_record(&self.header, &mut self.record) {
                Ok(0) => return None,
                Ok(_) => {
                    if self.record.flags().is_unmapped() {
                        return Some(Ok(self.record.clone()));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
    }
}
