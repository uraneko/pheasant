use pheasant_http::{ErrorStatus, err_stt};
use std::io::{Read, Seek, SeekFrom};

type ByteRange = [Option<usize>; 2];

pub struct Range {
    ranges: Vec<ByteRange>,
}

impl Range {
    pub fn new(value: &[u8]) -> Result<Self, ErrorStatus> {
        if !value.starts_with(b"bytes=") {
            return err_stt!(?400);
        }

        let capa = value.iter().filter(|b| **b == b',').count() + 1;
        let mut ranges = Vec::with_capacity(if capa == 0 { 1 } else { capa });

        let mut zero = 5;
        while let Some(idx) = value[zero + 1..].iter().position(|b| *b == b',') {
            ranges.push(parse_range(&value[zero + 1..])?);
            zero += idx;
        }

        if zero == 5 {
            ranges.push(parse_range(&value[6..])?);
        }

        Ok(Self { ranges })
    }

    pub fn write(
        &self,
        seeker: &mut (impl Seek + Read),
        buf: &mut Vec<u8>,
    ) -> Result<(), ErrorStatus> {
        for range in self.ranges.iter() {
            write_range(range, seeker, buf)?
        }

        Ok(())
    }
}

pub fn write_range(
    range: &[Option<usize>; 2],
    seeker: &mut (impl Seek + Read),
    buf: &mut Vec<u8>,
) -> Result<(), ErrorStatus> {
    match range {
        [Some(start), Some(end)] => {
            buf.resize(end - start, 0);
            seeker
                .seek(SeekFrom::Start(*start as u64))
                .map_err(|_| err_stt!(416))?;
            seeker.read(buf).map_err(|_| err_stt!(422))?;
        }
        [Some(start), None] => {
            seeker
                .seek(SeekFrom::Start(*start as u64))
                .map_err(|_| err_stt!(416))?;
            buf.clear();
            seeker.read_to_end(buf).map_err(|_| err_stt!(422))?;
        }
        [None, Some(end)] => {
            seeker
                .seek(SeekFrom::End(-(*end as i64)))
                .map_err(|_| err_stt!(416))?;
            buf.clear();
            seeker.read_to_end(buf).map_err(|_| err_stt!(422))?;
        }
        [None, None] => return err_stt!(?416),
    }

    Ok(())
}

// parses the range whatever its syntax
pub fn parse_range(range: &[u8]) -> Result<ByteRange, ErrorStatus> {
    match range {
        r if r.starts_with(b"-") => parse_start(range),
        r if r.contains(&b'-') => parse_full(range),
        _ => parse_end(range),
    }
}

// the range is only and end half
// bytes=-432
pub fn parse_end(range: &[u8]) -> Result<ByteRange, ErrorStatus> {
    if range[0] != b'-' {
        return err_stt!(?400);
    }

    let num = parse_num(&range[1..])?;

    Ok([None, Some(num)])
}

// the ramge is fully provided
// bytes=123-234
pub fn parse_full(range: &[u8]) -> Result<ByteRange, ErrorStatus> {
    let Some(pos) = range.iter().position(|b| *b == b'-') else {
        return err_stt!(?400);
    };

    let start = parse_num(&range[..pos])?;
    let end = parse_num(&range[pos + 1..])?;

    Ok([Some(start), Some(end)])
}

// the range is only a start half
// bytes=234-
pub fn parse_start(range: &[u8]) -> Result<ByteRange, ErrorStatus> {
    let len = range.len();
    if range[len - 1] != b'-' {
        return err_stt!(?400);
    }
    let num = parse_num(&range[..len - 1])?;

    Ok([Some(num), None])
}

pub fn parse_num(num: &[u8]) -> Result<usize, ErrorStatus> {
    let Ok(s) = str::from_utf8(num) else {
        return err_stt!(?400);
    };

    s.parse::<usize>().map_err(|_| err_stt!(400))
}
