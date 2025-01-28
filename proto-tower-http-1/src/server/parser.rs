use crate::data::HTTP1Request;
use http::{HeaderMap, HeaderName, HeaderValue, Method, Uri};
use parser_helper::ParseHelper;
use std::str::FromStr;

#[derive(Debug)]
pub enum Http1ParseError {}

const WHITESPACE: &[u8] = b" \t";
const COLON_WHITESPACE: &[u8] = b": \t";
const PATH_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~:/?#[]@!$&'()*+,;=%";

/// Parse request input, returning either a full error or the successful result or partial error
pub fn parse_request(input: &[u8]) -> Result<Result<HTTP1Request, (HTTP1Request, Vec<Http1ParseError>)>, &'static str> {
    let (header_block, body_block) = input.take_until_err(b"\r\n\r\n", "Expected header block to end with \\r\\n\\r\\n")?;
    let (_separator, body_block) = body_block.take_expect_err(b"\r\n\r\n", "Expected body block to start with \\r\\n")?;

    let ((method, path, _version), header_block) = get_request_line(header_block)?;
    let (header_map, _should_be_empty) = get_header_map(header_block)?;

    let body = body_block.to_vec();

    Ok(Ok(HTTP1Request {
        path,
        method,
        headers: header_map,
        body,
    }))
}

/// Get the first request line of an HTTP request
fn get_request_line(header_block: &[u8]) -> Result<((Method, Uri, &[u8]), &[u8]), &'static str> {
    let (_whitespace, header_block) = header_block
        .take_largest_err(|i| just_charset(i, WHITESPACE), 0, "Failed reading whitespace before method - should never happen")
        .unwrap();
    let (method, header_block) = get_method(header_block)?;
    let (_whitespace, header_block) = header_block.take_largest_err(|i| just_charset(i, WHITESPACE), 1, "Expected whitespace after method")?;
    let (path, header_block) = header_block.take_largest_err(|i| just_charset(i, PATH_CHARSET), 1, "We expected at least one character for the path")?;
    let path = std::str::from_utf8(path).map_err(|_| "Failed to parse path")?;
    let path = Uri::from_str(path).map_err(|_| "Failed to parse path")?;
    let (_whitespace, header_block) = header_block.take_largest_err(|i| just_charset(i, WHITESPACE), 1, "Expected whitespace after path")?;
    // TODO does not handle trailing whitespace and includes the \r\n
    let (version, header_block) = header_block.take_expect_err(b"HTTP/1.1", "Expected version to be HTTP/1.1 with \\r\\n after")?;
    let (_crlf, header_block) = header_block.maybe_expect(b"\r\n");
    Ok(((method, path, version), header_block))
}

/// Given a slice of JUST header bytes (excluding first HTTP request line and HTTP request trailing body), return a header map
fn get_header_map(mut header_block: &[u8]) -> Result<(HeaderMap, &[u8]), &'static str> {
    let mut header_map = HeaderMap::new();
    loop {
        match header_block.take_until(b"\r\n") {
            Ok((header_line, new_block)) => {
                let (_crlf, new_block) = new_block.take_expect_err(b"\r\n", "Expected CRLF at the end of the header line")?;
                header_block = new_block;
                let (h, v) = get_header_entry(header_line)?;
                header_map.insert(h, v);
            }
            Err(_) => {
                if !header_block.is_empty() {
                    let (h, v) = get_header_entry(header_block)?;
                    header_map.insert(h, v);
                }
                break;
            }
        }
    }
    Ok((header_map, header_block))
}

/// Given a slice of JUST header bytes in a single line, return a header entry
fn get_header_entry(header_line: &[u8]) -> Result<(HeaderName, HeaderValue), &'static str> {
    let (header, rest) = header_line.take_until_err(b":", "Expected a colon in the header line")?;
    let (_colon_whitespace, value) = rest.take_largest_err(|i| just_charset(i, COLON_WHITESPACE), 1, "Expected whitespace after colon")?;
    let h = HeaderName::from_bytes(header).map_err(|_| "Failed to parse header name")?;
    let v = std::str::from_utf8(value)
        .map_err(|_| "Failed to parse header value as string")?
        .parse()
        .map_err(|_| "Failed to parse header value as struct")?;
    Ok((h, v))
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

    #[test]
    fn get_no_headers_no_body_request() {
        let input = b"GET / HTTP/1.1\r\n\r\n";
        let res = parse_request(input);
        let res = res.unwrap();
        let res = res.unwrap();
        assert_eq!(res.method, http::Method::GET);
        assert_eq!(res.path, http::Uri::from_static("/"));
        assert_eq!(res.body, b"");
        assert_eq!(res.headers.len(), 0);
    }
}
