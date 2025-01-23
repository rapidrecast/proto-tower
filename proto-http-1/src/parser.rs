use crate::HTTP1Request;
use http::Uri;
use parser_helper::ParseHelper;
use std::str::FromStr;

#[derive(Debug)]
pub enum Http1ParseError {}

/// Parse request input, returning either a full error or the successful result or partial error
pub fn parse_request(input: &[u8]) -> Result<Result<HTTP1Request, (HTTP1Request, Vec<Http1ParseError>)>, &'static str> {
    const WHITESPACE: &[u8] = b" \t";
    const PATH_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~:/?#[]@!$&'()*+,;=%";
    let (_whitespace, input) = input.take_largest_err(|i| just_charset(i, WHITESPACE), 0, "Failed reading whitespace before method - should never happen").unwrap();
    let (method, input) = get_method(input)?;
    let (_whitespace, input) = input.take_largest_err(|i| just_charset(i, WHITESPACE), 1, "Expected whitespace after method")?;
    let (path, input) = input.take_largest_err(|i| just_charset(i, PATH_CHARSET), 1, "We expected at least one character for the path")?;
    let path = std::str::from_utf8(path).map_err(|_| "Failed to parse path")?;
    let path = Uri::from_str(path).map_err(|_| "Failed to parse path")?;
    let (_whitespace, input) = input.take_largest_err(|i| just_charset(i, WHITESPACE), 1, "Expected whitespace after path")?;
    let (_version, input) = input.take_expect_err(b"HTTP/1.1\r\n", "Expected version to be HTTP/1.1 with \\r\\n after")?;
    let (_header_block, input) = input.take_until_err(b"\r\n\r\n", "Expected header block to end with \\r\\n\\r\\n")?;
    let (_separator, body_block) = input.take_expect_err(b"\r\n\r\n", "Expected body block to start with \\r\\n")?;
    let body = body_block.to_vec();

    Ok(Ok(HTTP1Request {
        path,
        method,
        headers: Default::default(),
        body,
    }))
}

/// Read a method from the input and return remaining sequence
fn get_method(input: &[u8]) -> Result<(http::Method, &[u8]), &'static str> {
    let (method, rem) = input.take_smallest_err(|seq| is_method(seq), 1, "Expected a block-capital method")?;
    let method = match method {
        b"GET" => http::Method::GET,
        b"POST" => http::Method::POST,
        b"PUT" => http::Method::PUT,
        b"DELETE" => http::Method::DELETE,
        b"PATCH" => http::Method::PATCH,
        b"OPTIONS" => http::Method::OPTIONS,
        b"HEAD" => http::Method::HEAD,
        b"TRACE" => http::Method::TRACE,
        b"CONNECT" => http::Method::CONNECT,
        _ => return Err("Invalid method"),
    };
    Ok((method, rem))
}

/// True if the input starts with a method
fn is_method(input: &[u8]) -> bool {
    // TODO handle capitalization
    let methods = [
        "GET".as_bytes(),
        "POST".as_bytes(),
        "PUT".as_bytes(),
        "DELETE".as_bytes(),
        "PATCH".as_bytes(),
        "OPTIONS".as_bytes(),
        "HEAD".as_bytes(),
        "TRACE".as_bytes(),
        "CONNECT".as_bytes(),
    ];
    for method in methods.iter() {
        if input.len() < method.len() {
            continue;
        }
        if input[..method.len()] == method[..] {
            return true;
        }
    }
    false
}

/// True if the input is just the character set
fn just_charset(input: &[u8], charset: &[u8]) -> bool {
    for i in 0..input.len() {
        if !charset.contains(&input[i]) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::parse_request;

    #[test]
    fn can_parse_request() {
        let input = b"CONNECT /some/path?param=123&other=456 HTTP/1.1\r\nHost: localhost\r\n\r\nSome BODY";
        let res = parse_request(input);
        let res = res.unwrap();
        let res = res.unwrap();
        assert_eq!(res.method, http::Method::CONNECT);
        assert_eq!(res.path, http::Uri::from_static("/some/path?param=123&other=456"));
        assert_eq!(res.body, b"Some BODY");
    }
}