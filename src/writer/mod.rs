use std::io;

pub mod car;

pub mod splitter;
pub mod uploader;

#[cfg(feature = "encryption")]
pub mod crypto;

pub trait ChainWrite<W: io::Write>: io::Write {
    fn write2next(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.next_writer().write(buf)?;
        Ok(buf.len())
    }
    fn next_writer(&mut self) -> &mut W;
    fn next(self) -> W;
}

macro_rules! take_nth_writer {
    ($w:ident $($tails:tt)*) => {
        take_nth_writer!(@next(ChainWrite::next($w)) $($tails)*)
    };
    (@next($($x:tt)*) > $($tails:tt)*) => {
        // take_nth_next!(@next(Option::and_then($($x)*, |x| x.next())) $($tails)*)
        take_nth_writer!(@next(ChainWrite::next($($x)*)) $($tails)*)
    };
    (@next($($x:tt)*)) => {
        $($x)*
    };
}
pub(crate) use take_nth_writer;
