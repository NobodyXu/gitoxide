use gix_object::bstr::{BStr, ByteSlice};
use winnow::{combinator::alt, error::ParserError, prelude::*, token::take_while};

fn is_hex_digit_lc(b: u8) -> bool {
    matches!(b, b'0'..=b'9' | b'a'..=b'f')
}

/// Copy from https://github.com/Byron/gitoxide/blob/f270850ff92eab15258023b8e59346ec200303bd/gix-object/src/immutable/parse.rs#L64
pub fn hex_hash<'a, E: ParserError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E> {
    // NOTE: It's important to be able to read all hashes, do not parameterize it. Hashes can be rejected at a later stage
    // if needed.
    take_while(
        gix_hash::Kind::shortest().len_in_hex()..=gix_hash::Kind::longest().len_in_hex(),
        is_hex_digit_lc,
    )
    .parse_next(i)
    .map(|(i, hex)| (i, hex.as_bstr()))
}

pub fn newline<'a, E: ParserError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8], E> {
    alt((b"\r\n", b"\n")).parse_next(i)
}
