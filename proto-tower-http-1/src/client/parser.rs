use crate::data::HTTP1Response;
use parser_helper::ParseHelper;

pub fn parse_http1_response(buff: &[u8]) -> Result<HTTP1Response, &'static str> {
    // First line of response
    let (http_version, buff) = buff.take_until_err(" ".as_bytes(), "Expected a space in response")?;
    if http_version != "HTTP/1.1".as_bytes() {
        return Err("Expected HTTP/1.1");
    }
    let (_space, buff) = buff.take_expect_err(" ".as_bytes(), "Expected space")?;
    let (status_and_description, buff) = buff.take_until_err("\r\n".as_bytes(), "Expected new line")?;
    let (status, _description) = status_and_description.take_until_err(" ".as_bytes(), "Expected space in status")?;
    let status = http::StatusCode::from_bytes(status).map_err(|_| "Expected status code deserializable")?;
    let (_new_line, mut buff) = buff.take_expect_err("\r\n".as_bytes(), "Expected new line")?;
    // Headers
    let mut headers = http::HeaderMap::new();
    loop {
        let (next_header, new_buff) = buff.take_until_err("\r\n".as_bytes(), "Expected new line")?;
        let (_new_line, new_buff) = new_buff.take_expect_err("\r\n".as_bytes(), "Expected new line")?;
        buff = new_buff;
        if next_header.is_empty() {
            break;
        }
        let (header_name, next_header) = next_header.take_until_err(": ".as_bytes(), "Expected header name before colon space")?;
        let (_colon_space, header_value) = next_header.take_expect_err(": ".as_bytes(), "Expected colon space in headers")?;
        let header_name = http::header::HeaderName::from_bytes(header_name).map_err(|_| "Expected header name to be deserializable")?;
        let header_value = http::header::HeaderValue::from_bytes(header_value).map_err(|_| "Expected header value to be deserializable")?;
        headers.insert(header_name, header_value);
    }
    // We could read exact, but that is too much hassle
    let body = buff.to_vec();

    Ok(HTTP1Response { status, headers, body })
}
