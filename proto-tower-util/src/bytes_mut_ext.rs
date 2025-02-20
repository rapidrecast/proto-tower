use bytes::{Bytes, BytesMut};
use std::ops::Range;

pub trait BytesMutHelper {
    fn safe_peek(&self, range: std::ops::Range<usize>) -> Option<Bytes>;
}

impl BytesMutHelper for BytesMut {
    /// TODO this may be deprecated in favour of BytesMut::get - check unused
    fn safe_peek(&self, range: Range<usize>) -> Option<Bytes> {
        if range.end > self.len() {
            return None;
        }
        Some(Bytes::copy_from_slice(&self[range]))
    }
}
