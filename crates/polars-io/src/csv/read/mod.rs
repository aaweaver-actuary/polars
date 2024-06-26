//! Functionality for reading CSV files.
//!
//! # Examples
//!
//! ```
//! use polars_core::prelude::*;
//! use polars_io::prelude::*;
//! use std::fs::File;
//!
//! fn example() -> PolarsResult<DataFrame> {
//!     // Prefer `from_path` over `new` as it is faster.
//!     CsvReadOptions::default()
//!         .with_has_header(true)
//!         .try_into_reader_with_file_path(Some("example.csv".into()))?
//!         .finish()
//! }
//! ```

mod buffer;
mod options;
mod parser;
mod read_impl;
mod reader;
mod splitfields;
mod utils;

pub use options::{CommentPrefix, CsvEncoding, CsvParseOptions, CsvReadOptions, NullValues};
pub use parser::count_rows;
pub use read_impl::batched_mmap::{BatchedCsvReaderMmap, OwnedBatchedCsvReaderMmap};
pub use read_impl::batched_read::{BatchedCsvReaderRead, OwnedBatchedCsvReader};
pub use reader::CsvReader;
pub use utils::{infer_file_schema, is_compressed};
