//! Creates a new BAM file.
//!
//! This writes a SAM header, reference sequences, and one unmapped record to stdout.
//!
//! Verify the output by piping to `samtools view --no-PG --with-header`.

use std::io;

use noodles_bam as bam;
use noodles_sam::{
    self as sam,
    alignment::Record,
    header::record::value::{map::Program, Map},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout().lock();
    let mut writer = bam::Writer::new(stdout);

    let header = sam::Header::builder()
        .set_header(Default::default())
        .add_program("noodles-bam", Map::<Program>::default())
        .add_comment("an example BAM written by noodles-bam")
        .build();

    writer.write_header(&header)?;

    let record = Record::default();
    writer.write_record(&header, &record)?;

    Ok(())
}
