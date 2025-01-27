# Proto-http-2

H2 is the HTTP/2 protocol over TLS.
This is the most common way to use HTTP/2.

H2C is the HTTP/2 protocol without the TLS layer.
This is useful for when you want to use HTTP/2 but don't want to deal with the complexity of TLS.

The TLS layer can be injected separately.
This crate handles the HTTP/2 protocol with or without TLS.

https://www.rfc-editor.org/rfc/rfc7540

## Adding dependency to project

```toml
[dependencies]
proto-http-2 = { git = "https://github.com/rapidrecast/proto-tower.git", subdir = "proto-http-2" }
```


# Notes

If you are upgrading to http 2 from http 1, you should NOT do that if the upgrade protocol is h2c.

From the [RFC](https://tools.ietf.org/html/rfc7540#section-3.2):