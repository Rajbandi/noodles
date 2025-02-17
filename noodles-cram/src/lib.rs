#![warn(missing_docs)]

//! **noodles-cram** handles the reading and writing of the CRAM format.

#[cfg(feature = "async")]
pub mod r#async;

pub mod codecs;
pub(crate) mod container;
pub mod crai;
pub mod data_container;
pub mod file_definition;
mod huffman;
pub mod indexed_reader;
mod indexer;
pub(crate) mod io;
mod num;
pub mod reader;
pub mod record;
pub mod writer;

pub use self::{
    data_container::DataContainer, file_definition::FileDefinition, indexed_reader::IndexedReader,
    indexer::index, reader::Reader, record::Record, writer::Writer,
};

#[cfg(feature = "async")]
pub use self::r#async::{Reader as AsyncReader, Writer as AsyncWriter};

static MAGIC_NUMBER: &[u8] = b"CRAM";
