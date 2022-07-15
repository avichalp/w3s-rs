//! This module is from [https://github.com/n0-computer/iroh](https://github.com/n0-computer/iroh).
//! Since this module is not published to crate.io, I can only import it here and did a little modification.
//! 
//! The writer part has been changed to sync io.
//! 
//! 
//! Implementation of the [car](https://ipld.io/specs/transport/car/) format.

pub mod error;
mod header;
mod reader;
mod util;
mod writer;

pub use header::CarHeader;
pub use reader::CarReader;
pub use writer::CarWriter;
