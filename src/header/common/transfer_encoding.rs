use header::{Header, HeaderFormat};
use std::fmt;
use std::str::FromStr;
use header::shared::util::{from_comma_delimited, fmt_comma_delimited};

use self::Encoding::{Chunked, Gzip, Deflate, Compress, EncodingExt};

/// The `Transfer-Encoding` header.
///
/// This header describes the encoding of the message body. It can be
/// comma-separated, including multiple encodings.
///
/// ```notrust
/// Transfer-Encoding: gzip, chunked
/// ```
///
/// According to the spec, if a `Content-Length` header is not included,
/// this header should include `chunked` as the last encoding.
///
/// The implementation uses a vector of `Encoding` values.
#[derive(Clone, PartialEq, Show)]
pub struct TransferEncoding(pub Vec<Encoding>);

deref!(TransferEncoding => Vec<Encoding>);

/// A value to be used with the `Transfer-Encoding` header.
///
/// Example:
///
/// ```
/// # use hyper::header::TransferEncoding;
/// # use hyper::header::transfer_encoding::Encoding::{Gzip, Chunked};
/// # use hyper::header::Headers;
/// # let mut headers = Headers::new();
/// headers.set(TransferEncoding(vec![Gzip, Chunked]));
#[derive(Clone, PartialEq, Show)]
pub enum Encoding {
    /// The `chunked` encoding.
    Chunked,
    /// The `gzip` encoding.
    Gzip,
    /// The `deflate` encoding.
    Deflate,
    /// The `compress` encoding.
    Compress,
    /// Some other encoding that is less common, can be any String.
    EncodingExt(String)
}

impl fmt::String for Encoding {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", match *self {
            Chunked => "chunked",
            Gzip => "gzip",
            Deflate => "deflate",
            Compress => "compress",
            EncodingExt(ref s) => s.as_slice()
        })
    }
}

impl FromStr for Encoding {
    fn from_str(s: &str) -> Option<Encoding> {
        match s {
            "chunked" => Some(Chunked),
            "deflate" => Some(Deflate),
            "gzip" => Some(Gzip),
            "compress" => Some(Compress),
            _ => Some(EncodingExt(s.to_string()))
        }
    }
}

impl Header for TransferEncoding {
    fn header_name(_: Option<TransferEncoding>) -> &'static str {
        "Transfer-Encoding"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Option<TransferEncoding> {
        from_comma_delimited(raw).map(TransferEncoding)
    }
}

impl HeaderFormat for TransferEncoding {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt_comma_delimited(fmt, &self[])
    }
}

bench_header!(normal, TransferEncoding, { vec![b"chunked, gzip".to_vec()] });
bench_header!(ext, TransferEncoding, { vec![b"ext".to_vec()] });
