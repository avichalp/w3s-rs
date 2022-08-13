use std::io;

pub mod car;

pub mod splitter;
pub mod uploader;
pub mod downloader;

#[cfg(feature = "encryption")]
pub mod crypto;

#[cfg(feature = "zstd")]
pub mod decompressor;

pub trait ChainWrite<W: io::Write>: io::Write {
    fn next_writer(&mut self) -> &mut W;
    fn next(self) -> W;
}

#[macro_export(local_inner_macros)]
macro_rules! take_nth_writer {
    ($w:ident $($tails:tt)*) => {
        take_nth_writer!(@next($crate::writer::ChainWrite::next($w)) $($tails)*)
    };
    (@next($($x:tt)*) > $($tails:tt)*) => {
        // take_nth_next!(@next(Option::and_then($($x)*, |x| x.next())) $($tails)*)
        take_nth_writer!(@next($crate::writer::ChainWrite::next($($x)*)) $($tails)*)
    };
    (@next($($x:tt)*)) => {
        $($x)*
    };
}
pub use take_nth_writer;
