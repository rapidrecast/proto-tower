use bytes::Bytes;
use kafka_protocol::messages::ApiKey;
use kafka_protocol::protocol::buf::{ByteBuf, NotEnoughBytesError};
use std::collections::BTreeMap;

/// The header of the response can be unpredictable, either containing or not containing tagged fields
#[derive(Default)]
pub struct ResponseHeaderIntermediary {
    /// The correlation ID of this response.
    pub correlation_id: i32,
}

impl ResponseHeaderIntermediary {
    pub fn decode<B: ByteBuf>(buf: &mut B) -> Result<Self, NotEnoughBytesError> {
        let correlation_id = ByteBuf::try_get_i32(buf)?;
        Ok(Self { correlation_id })
    }
    pub fn with_correlation_id(mut self, value: i32) -> Self {
        self.correlation_id = value;
        self
    }

    pub fn complete<B: ByteBuf>(self, buf: &mut B, api: ApiKey, api_version: i16) -> Result<ResponseHeaderComplete, NotEnoughBytesError> {
        let mut unknown_tagged_fields = BTreeMap::new();
        if self.tagged_fields(api, api_version) {
            let num_tagged_fields = decode_unsigned_varint(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = decode_unsigned_varint(buf)?;
                let size: u32 = decode_unsigned_varint(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(ResponseHeaderComplete {
            correlation_id: self.correlation_id,
            tagged_fields: unknown_tagged_fields,
        })
    }

    fn tagged_fields(&self, api: ApiKey, _api_version: i16) -> bool {
        if api == ApiKey::ApiVersions {
            false
        } else {
            true
        }
    }
}

fn decode_unsigned_varint<B: ByteBuf>(buf: &mut B) -> Result<u32, NotEnoughBytesError> {
    let mut value = 0;
    for i in 0..5 {
        let b = ByteBuf::try_get_u8(buf)? as u32;
        value |= (b & 0x7F) << (i * 7);
        if b < 0x80 {
            break;
        }
    }
    Ok(value.into())
}

pub struct ResponseHeaderComplete {
    /// The correlation ID of this response.
    pub correlation_id: i32,
    /// The tagged fields of this response.
    pub tagged_fields: BTreeMap<i32, Bytes>,
}
