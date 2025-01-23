# Proto-http-2

H2 is the HTTP/2 protocol over TLS.
This is the most common way to use HTTP/2.

H2C is the HTTP/2 protocol without the TLS layer.
This is useful for when you want to use HTTP/2 but don't want to deal with the complexity of TLS.

The TLS layer can be injected separately.
This crate handles the HTTP/2 protocol with or without TLS.

https://www.rfc-editor.org/rfc/rfc7540
