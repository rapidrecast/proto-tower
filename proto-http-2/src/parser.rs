use crate::Http2Request;

#[derive(Debug)]
pub enum H2CParseErrors {}

pub(crate) fn parse_request(p0: &Vec<u8>) -> Result<Result<Http2Request, (Http2Request, Vec<H2CParseErrors>)>, &'static str> {
    todo!()
}