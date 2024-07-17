//! Logic for handling `.lz` file format.
//!
//! Format specifications are at <https://www.nongnu.org/lzip/manual/lzip_manual.html#File-format> or [draft-diaz-lzip-09].
//!
//! [draft-diaz-lzip-09]: https://datatracker.ietf.org/doc/html/draft-diaz-lzip-09#section-2

pub(crate) mod crc;
pub(crate) mod header;
