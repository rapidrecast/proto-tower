use crate::layer::HTTP1Request;
use parser_helper::ParseHelper;

#[derive(Debug)]
pub enum Http1ParseError {

}

pub fn parse_request(input: &[u8]) -> Result<Result<HTTP1Request, (HTTP1Request, Vec<Http1ParseError>)>, &'static str> {
    let (_whitespace, input) = input.take_largest_err(|i| just_charset(i, b" \t"), 0, "We should never error since 0-size is valid").unwrap();
    let (method, input) = get_method(input)?;
    Ok(Ok(HTTP1Request {
        path: Default::default(),
        method,
        headers: Default::default(),
        body: vec![],
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
    Ok ((method, rem))
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
    if input.len() == 0 {
        return true;
    }
    for i in 0..charset.len() {
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
        let input = b"CONNECT / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let res = parse_request(input);
        let res = res.unwrap();
        let res = res.unwrap();
        assert_eq!(res.method, http::Method::CONNECT);
    }
}